import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { BENCH_DIR, REPO_DIR, RUNS, TOOLS_IMAGE, docker, log, sh, sleep, timed, waitForHttp } from './lib.mjs'

const WORK_DIR = mkdtempSync(join(tmpdir(), 'rust-starter-bench-'))
const HMR_RUNS = 10
const BUILDER = 'bench-builder'

const STACKS = {
  rust: {
    dir: `${WORK_DIR}/repo`,
    port: 9091,
    heading: 'Rust starter',
    markupFile: 'src/pages/home.rs',
    logicFile: 'src/server/mod.rs',
    logicFrom: '"ok"',
    logic: run => `"ok-${run}"`,
    check: ['./scripts/check.sh'],
    touchFile: 'src/main.rs',
    baseImages: ['rust:1.99-trixie', 'debian:trixie-slim'],
  },
  nuxt: {
    dir: `${WORK_DIR}/repo/bench/nuxt`,
    port: 9092,
    heading: 'Nuxt starter',
    markupFile: 'app/pages/index.vue',
    logicFile: 'server/routes/health.get.ts',
    logicFrom: '\'ok\'',
    logic: run => `'ok-${run}'`,
    check: ['npm', 'run', 'check'],
    touchFile: 'app/app.vue',
    baseImages: ['node:26-trixie-slim'],
  },
}

const composeArgs = app => ['compose', '-p', `bench-dev-${app}`, '-f', `${STACKS[app].dir}/compose.yaml`, '-f', `${BENCH_DIR}/dev-overrides/${app}.yaml`]
const compose = (app, ...args) => docker(...composeArgs(app), ...args)
const pageUrl = app => `http://localhost:${STACKS[app].port}/`

function freshClone() {
  rmSync(WORK_DIR, { recursive: true, force: true })
  sh('git', ['clone', '--quiet', '--branch', sh('git', ['-C', REPO_DIR, 'branch', '--show-current']), REPO_DIR, `${WORK_DIR}/repo`])
}

async function firstRun(app) {
  compose(app, 'down', '--volumes', '--remove-orphans')
  return timed(async () => {
    compose(app, 'build', '--no-cache', 'app')
    compose(app, 'up', '-d')
    await waitForHttp(pageUrl(app), { body: STACKS[app].heading })
  })
}

async function restart(app) {
  return timed(async () => {
    compose(app, 'restart', 'app')
    await waitForHttp(pageUrl(app), { body: STACKS[app].heading })
  })
}

function markupChange(app) {
  const stack = STACKS[app]
  const output = docker('run', '--rm', '--network', 'host', '-v', `${stack.dir}:/work`, '-v', `${BENCH_DIR}/scripts:/tools/scripts:ro`,
    TOOLS_IMAGE, 'node', '/tools/scripts/hmr.mjs', pageUrl(app), `/work/${stack.markupFile}`, stack.heading, String(HMR_RUNS))
  return JSON.parse(output.split('\n').at(-1))
}

async function logicChange(app) {
  const stack = STACKS[app]
  const file = `${stack.dir}/${stack.logicFile}`
  const original = readFileSync(file, 'utf8')
  const samples = []
  try {
    for (let run = 1; run <= RUNS; run++) {
      const before = readFileSync(file, 'utf8')
      const from = run === 1 ? stack.logicFrom : stack.logic(run - 1)
      samples.push(await timed(async () => {
        writeFileSync(file, before.replace(from, stack.logic(run)))
        await waitForHttp(`http://localhost:${stack.port}/health`, { body: `ok-${run}` })
      }))
      await sleep(2000)
    }
  }
  finally {
    writeFileSync(file, original)
  }
  await waitForHttp(`http://localhost:${stack.port}/health`, { body: 'ok', exact: true })
  return samples
}

function removeBuilder() {
  if (docker('buildx', 'ls', '--format', '{{.Name}}').split('\n').includes(BUILDER)) {
    docker('buildx', 'rm', '--force', BUILDER)
  }
}

async function imageBuild(app) {
  const stack = STACKS[app]
  const build = () => docker('buildx', 'build', '--builder', BUILDER, '--target', 'prod', '--output', 'type=cacheonly', stack.dir)
  removeBuilder()
  docker('buildx', 'create', '--name', BUILDER, '--driver', 'docker-container')
  sh('docker', ['buildx', 'build', '--builder', BUILDER, '--output', 'type=cacheonly', '-'],
    { input: stack.baseImages.map(image => `FROM ${image}`).join('\n') + '\n', stdio: ['pipe', 'pipe', 'pipe'] })
  const cold = await timed(async () => build())
  const touched = `${stack.dir}/${stack.touchFile}`
  const original = readFileSync(touched, 'utf8')
  writeFileSync(touched, `${original}\n`)
  const warm = await timed(async () => build())
  writeFileSync(touched, original)
  removeBuilder()
  return { cold, warm }
}

async function alternating(apps, label, measure) {
  for (let run = 0; run < RUNS; run++) {
    for (const app of run % 2 ? [...apps].reverse() : apps) {
      log(`${label} ${app} (${run + 1}/${RUNS})`)
      await measure(app)
    }
  }
}

export async function devloop() {
  docker('build', '-t', TOOLS_IMAGE, `${BENCH_DIR}/tools`)
  freshClone()
  const apps = (process.env.BENCH_APPS ?? 'rust,nuxt').split(',')
  const results = Object.fromEntries(apps.map(app => [app, { firstRunMs: [], restartMs: [], checksMs: [], imageBuildMs: { cold: [], warm: [] } }]))
  await alternating(apps, 'first run', async app => results[app].firstRunMs.push(await firstRun(app)))
  await alternating(apps, 'restart', async app => results[app].restartMs.push(await restart(app)))
  for (const app of apps) {
    log(`markup change ${app}`)
    results[app].markupChange = markupChange(app)
  }
  for (const app of [...apps].reverse()) {
    log(`logic change ${app}`)
    results[app].logicChangeMs = await logicChange(app)
  }
  for (const app of apps) {
    compose(app, 'exec', '-T', 'app', ...STACKS[app].check)
  }
  await alternating(apps, 'checks', async app => results[app].checksMs.push(await timed(async () => compose(app, 'exec', '-T', 'app', ...STACKS[app].check))))
  for (const app of apps) {
    compose(app, 'down', '--volumes', '--remove-orphans')
  }
  await alternating(apps, 'image build', async (app) => {
    const { cold, warm } = await imageBuild(app)
    results[app].imageBuildMs.cold.push(cold)
    results[app].imageBuildMs.warm.push(warm)
  })
  rmSync(WORK_DIR, { recursive: true, force: true })
  return results
}
