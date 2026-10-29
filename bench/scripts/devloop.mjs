import { randomUUID } from 'node:crypto'
import { readFileSync, writeFileSync } from 'node:fs'
import { client, devApp, signUp } from './parity.mjs'
import { BENCH_DIR, RUNS, TOOLS_IMAGE, docker, log, sh, sleep, timed, waitForHttp } from './lib.mjs'
import { DEVLOOP, IMAGE_BUILDS, VARIANTS, checkout } from './variants.mjs'

const HMR_RUNS = 10
const OFFSET = 1000
const countText = count => `· ${new Intl.NumberFormat('en').format(count)} notes`
const BUILDER = 'bench-builder'
const LOGIC_TIMEOUT_MS = 600_000

const KINDS = {
  rust: {
    heading: 'Rust starter',
    markupFile: 'src/pages/home.rs',
    logicFile: 'src/server/notes.rs',
    logic: offset => offset ? `        total: counts.number_of_items + ${offset},` : '        total: counts.number_of_items,',
    uiFile: 'src/ui/format.rs',
    uiLogic: offset => offset ? `    (value + ${offset}).to_formatted_string(&Locale::en)` : '    value.to_formatted_string(&Locale::en)',
    check: ['./scripts/check.sh'],
    touchFile: 'src/main.rs',
    containerPort: 8080,
    originEnv: 'PUBLIC_URL',
    baseImages: ['rust:1.99-trixie', 'debian:trixie-slim'],
  },
  nuxt: {
    heading: 'Nuxt starter',
    markupFile: 'app/pages/index.vue',
    logicFile: 'server/utils/notes.ts',
    logic: offset => offset ? `  const total = (totals?.total ?? 0) + ${offset}` : '  const total = totals?.total ?? 0',
    uiFile: 'app/utils/format.ts',
    uiLogic: offset => `export const formatCount = (value: number) => countFormat.format(${offset ? `value + ${offset}` : 'value'})`,
    check: ['npm', 'run', 'check'],
    touchFile: 'app/app.vue',
    containerPort: 3000,
    originEnv: 'NUXT_SITE_ORIGIN',
    baseImages: ['node:26-trixie-slim'],
  },
}

const kind = variant => KINDS[VARIANTS[variant].kind]
const pageUrl = variant => `http://localhost:${VARIANTS[variant].devPort}/`

function overrideFile(variant) {
  const { devPort } = VARIANTS[variant]
  const file = `${checkout(variant)}/compose.bench.yaml`
  writeFileSync(file, `services:\n  app:\n    ports: !override\n      - "${devPort}:${kind(variant).containerPort}"\n    environment:\n      ${kind(variant).originEnv}: http://localhost:${devPort}\n`)
  return file
}

const compose = (variant, ...args) => docker('compose', '-p', `bench-dev-${variant}`, '-f', `${checkout(variant)}/compose.yaml`, '-f', overrideFile(variant), ...args)

async function firstRun(variant) {
  compose(variant, 'down', '--volumes', '--remove-orphans')
  return timed(async () => {
    compose(variant, 'build', '--no-cache', 'app')
    compose(variant, 'up', '-d')
    await waitForHttp(pageUrl(variant), { body: kind(variant).heading })
  })
}

async function restart(variant) {
  return timed(async () => {
    compose(variant, 'restart', 'app')
    await waitForHttp(pageUrl(variant), { body: kind(variant).heading })
  })
}

function editVisible(variant, spec) {
  const output = docker('run', '--rm', '--network', 'host', '-v', `${checkout(variant)}:/work`, '-v', `${BENCH_DIR}/scripts:/tools/scripts:ro`,
    TOOLS_IMAGE, 'node', '/tools/scripts/edit-visible.mjs', JSON.stringify({ ...spec, file: `/work/${spec.file}` }))
  return JSON.parse(output.split('\n').at(-1))
}

function markupChange(variant) {
  const { heading, markupFile } = kind(variant)
  const label = run => run ? `${heading} ${run}` : heading
  return editVisible(variant, {
    url: pageUrl(variant),
    file: markupFile,
    ready: heading,
    steps: Array.from({ length: HMR_RUNS }, (_, index) => ({ from: label(index), to: label(index + 1), expect: label(index + 1) })),
  })
}

async function uiLogicChange(variant) {
  const { uiFile, uiLogic } = kind(variant)
  const session = await signUp(devApp(variant), `devloop-${randomUUID()}@example.com`, randomUUID())
  const created = await client(devApp(variant), session.pair)('POST', '/api/notes', devApp(variant).note({ title: 'Dev loop note', body: '' }))
  if (created.status !== 200) throw new Error(`note creation failed with ${created.status}`)
  return editVisible(variant, {
    url: `${pageUrl(variant)}notes?page=1`,
    file: uiFile,
    cookie: session.pair,
    ready: countText(1),
    steps: Array.from({ length: RUNS }, (_, index) => ({ from: uiLogic(index * OFFSET), to: uiLogic((index + 1) * OFFSET), expect: countText((index + 1) * OFFSET + 1) })),
  })
}

async function waitForTotal(http, expected) {
  const start = Date.now()
  while (Date.now() - start < LOGIC_TIMEOUT_MS) {
    try {
      if ((await http('GET', '/api/notes?page=1')).json?.total === expected) return
    }
    catch {}
    await sleep(50)
  }
  throw new Error(`total never became ${expected}`)
}

async function logicChange(variant) {
  const file = `${checkout(variant)}/${kind(variant).logicFile}`
  const original = readFileSync(file, 'utf8')
  if (!original.includes(kind(variant).logic(0))) throw new Error(`${file} does not contain the logic line`)
  const session = await signUp(devApp(variant), `devloop-${randomUUID()}@example.com`, randomUUID())
  const http = client(devApp(variant), session.pair)
  await waitForTotal(http, 0)
  const samples = []
  try {
    for (let run = 1; run <= RUNS; run++) {
      const before = readFileSync(file, 'utf8')
      samples.push(await timed(async () => {
        writeFileSync(file, before.replace(kind(variant).logic((run - 1) * OFFSET), kind(variant).logic(run * OFFSET)))
        await waitForTotal(http, run * OFFSET)
      }))
      await sleep(2000)
    }
  }
  finally {
    writeFileSync(file, original)
  }
  await waitForTotal(http, 0)
  return samples
}

function removeBuilder() {
  if (docker('buildx', 'ls', '--format', '{{.Name}}').split('\n').includes(BUILDER)) {
    docker('buildx', 'rm', '--force', BUILDER)
  }
}

async function imageBuild(variant) {
  const dir = checkout(variant)
  const build = () => docker('buildx', 'build', '--builder', BUILDER, '--target', 'prod', '--output', 'type=cacheonly', dir)
  removeBuilder()
  docker('buildx', 'create', '--name', BUILDER, '--driver', 'docker-container')
  sh('docker', ['buildx', 'build', '--builder', BUILDER, '--output', 'type=cacheonly', '-'],
    { input: kind(variant).baseImages.map(image => `FROM ${image}`).join('\n') + '\n', stdio: ['pipe', 'pipe', 'pipe'] })
  const cold = await timed(async () => build())
  const touched = `${dir}/${kind(variant).touchFile}`
  const original = readFileSync(touched, 'utf8')
  writeFileSync(touched, `${original}\n`)
  const warm = await timed(async () => build())
  writeFileSync(touched, original)
  removeBuilder()
  return { cold, warm }
}

async function alternating(variants, label, measure) {
  for (let run = 0; run < RUNS; run++) {
    for (const variant of run % 2 ? [...variants].reverse() : variants) {
      log(`${label} ${variant} (${run + 1}/${RUNS})`)
      await measure(variant)
    }
  }
}

export async function devloop() {
  docker('build', '-t', TOOLS_IMAGE, `${BENCH_DIR}/tools`)
  const results = Object.fromEntries(DEVLOOP.map(variant => [variant, { firstRunMs: [], restartMs: [], checksMs: [] }]))
  await alternating(DEVLOOP, 'first run', async variant => results[variant].firstRunMs.push(await firstRun(variant)))
  await alternating(DEVLOOP, 'restart', async variant => results[variant].restartMs.push(await restart(variant)))
  for (const variant of DEVLOOP) {
    log(`markup change ${variant}`)
    results[variant].markupChange = markupChange(variant)
    log(`browser logic change ${variant}`)
    results[variant].uiLogicChange = await uiLogicChange(variant)
  }
  for (const variant of [...DEVLOOP].reverse()) {
    log(`logic change ${variant}`)
    results[variant].logicChangeMs = await logicChange(variant)
  }
  for (const variant of DEVLOOP) compose(variant, 'exec', '-T', 'app', ...kind(variant).check)
  await alternating(DEVLOOP, 'checks', async variant => results[variant].checksMs.push(await timed(async () => compose(variant, 'exec', '-T', 'app', ...kind(variant).check))))
  for (const variant of DEVLOOP) compose(variant, 'down', '--volumes', '--remove-orphans')
  const builds = IMAGE_BUILDS.filter(variant => DEVLOOP.includes(variant))
  for (const variant of builds) results[variant].imageBuildMs = { cold: [], warm: [] }
  await alternating(builds, 'image build', async (variant) => {
    const { cold, warm } = await imageBuild(variant)
    results[variant].imageBuildMs.cold.push(cold)
    results[variant].imageBuildMs.warm.push(warm)
  })
  return results
}
