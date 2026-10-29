import { readFileSync, writeFileSync } from 'node:fs'
import os from 'node:os'
import { BENCH_DIR, docker } from './lib.mjs'
import { VARIANTS } from './variants.mjs'

const median = values => {
  const sorted = [...values].sort((a, b) => a - b)
  const middle = Math.floor(sorted.length / 2)
  return sorted.length % 2 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2
}
const number = new Intl.NumberFormat('en', { maximumFractionDigits: 1 })
const seconds = ms => `${number.format(ms / 1000)} s`
const millis = ms => `${number.format(ms)} ms`
const mib = value => `${number.format(value)} MiB`
const megabytes = value => `${number.format(value / 1_000_000)} MB`
const stat = (values, format) => values.length > 1
  ? `${format(median(values))} (${format(Math.min(...values))} to ${format(Math.max(...values))})`
  : format(values[0])

function table(results, label, rows) {
  const variants = Object.keys(results)
  const header = `| ${label} | ${variants.map(variant => VARIANTS[variant].label).join(' | ')} | Best |`
  const divider = `|---|${variants.map(() => '---').join('|')}|---|`
  const lines = rows.map(({ name, values, format, higherIsBetter = false }) => {
    const present = variants.filter(variant => values(results[variant])?.length)
    const best = present.reduce((a, b) => (higherIsBetter ? median(values(results[b])) > median(values(results[a])) : median(values(results[b])) < median(values(results[a]))) ? b : a)
    const cells = variants.map(variant => values(results[variant])?.length ? stat(values(results[variant]), format) : 'not measured')
    return `| ${name} | ${cells.join(' | ')} | ${VARIANTS[best].label} |`
  })
  return [header, divider, ...lines].join('\n')
}

function featureSection(features, production) {
  const items = Object.keys(Object.values(production)[0].parity)
  const variants = Object.keys(production)
  return `## Features

${table(features, 'Metric', [
  { name: 'Hand-written code (lines)', values: r => [r.codeLines], format: number.format },
  { name: 'Test code (lines)', values: r => [r.testLines], format: number.format },
  { name: 'Tests', values: r => [r.testCount], format: number.format, higherIsBetter: true },
  { name: 'Generated code (lines)', values: r => [r.generatedLines], format: number.format },
  { name: 'Config (lines)', values: r => [r.configLines], format: number.format },
  { name: 'Direct dependencies', values: r => [r.dependencies.direct], format: number.format },
  { name: 'All dependencies (lockfile)', values: r => [r.dependencies.total], format: number.format },
])}

Code is counted with [scc](https://github.com/boyter/scc), without tests, generated stock components, generated
migrations or theme CSS. Each variant is counted from its own branch; the 0.8 branch moves the app into the binary
crate and makes seeding a subcommand, which hot-patching needs.

### Parity checklist

| Check | ${variants.map(variant => VARIANTS[variant].label).join(' | ')} |
|---|${variants.map(() => '---').join('|')}|
${items.map(item => `| ${item} | ${variants.map(variant => production[variant].parity[item] ? 'pass' : 'FAIL').join(' | ')} |`).join('\n')}`
}

function reloads(devloop, metric) {
  return Object.entries(devloop).map(([variant, r]) =>
    `${VARIANTS[variant].label} ${r[metric].filter(sample => sample.reloaded).length} of ${r[metric].length}`).join(', ')
}

function devloopSection(devloop) {
  return `## Developer loop

${table(devloop, 'Metric', [
  { name: 'First run (clone to first page, cold caches)', values: r => r.firstRunMs, format: seconds },
  { name: 'Restart (warm caches)', values: r => r.restartMs, format: seconds },
  { name: 'Markup change visible in the browser', values: r => r.markupChange.map(sample => sample.ms), format: millis },
  { name: 'Browser logic change visible (count formatter)', values: r => r.uiLogicChange.map(sample => sample.ms), format: seconds },
  { name: 'Server logic change served by the API', values: r => r.logicChangeMs, format: seconds },
  { name: 'Lint, typecheck and tests (warm)', values: r => r.checksMs, format: seconds },
  { name: 'Production image build, cold', values: r => r.imageBuildMs?.cold, format: seconds },
  { name: 'Production image build, after a one-line change', values: r => r.imageBuildMs?.warm, format: seconds },
])}

Full page reloads during markup changes: ${reloads(devloop, 'markupChange')}. During browser logic changes:
${reloads(devloop, 'uiLogicChange')}.`
}

function productionSection(production) {
  const paths = Object.keys(Object.values(production)[0].load)
  const load = (path, mode, key) => r => r.load[path].map(run => run[mode][key])
  const lighthouse = (page, key) => r => r.lighthouse[page].map(run => run[key])
  return `## Production

${table(production, 'Metric', [
  { name: 'Image size', values: r => [r.imageBytes], format: megabytes },
  { name: 'Cold start to healthy (incl. migrations)', values: r => r.coldStartMs, format: millis },
  { name: 'Memory, idle', values: r => r.idleMemoryMiB, format: mib },
  { name: 'Memory, peak under load', values: r => paths.flatMap(path => r.load[path].map(run => run.peakMemoryMiB)), format: mib },
  { name: 'Sign-in p50 (password hashing)', values: r => r.signIn.map(run => run.p50), format: millis },
])}

### Throughput (50 connections, 20 s runs after a 5 s warm-up)

${table(production, 'Endpoint', paths.map(path => ({ name: `\`${path}\` requests/s`, values: load(path, 'throughput', 'rps'), format: number.format, higherIsBetter: true })))}

### Latency at a fixed 50 requests/s (latency corrected, below every app's capacity)

${table(production, 'Endpoint', paths.flatMap(path => [
  { name: `\`${path}\` p50`, values: load(path, 'fixedRate', 'p50'), format: millis },
  { name: `\`${path}\` p99`, values: load(path, 'fixedRate', 'p99'), format: millis },
]))}

### Lighthouse (mobile preset, 5 runs)

${table(production, 'Page', ['signIn', 'notes'].flatMap(page => [
  { name: `${page} score`, values: lighthouse(page, 'score'), format: number.format, higherIsBetter: true },
  { name: `${page} LCP`, values: lighthouse(page, 'lcpMs'), format: millis },
  { name: `${page} total blocking time`, values: lighthouse(page, 'tbtMs'), format: millis },
  { name: `${page} bytes transferred`, values: lighthouse(page, 'totalBytes'), format: megabytes },
]))}`
}

export function report(resultsDir) {
  const load = name => JSON.parse(readFileSync(`${resultsDir}/${name}.json`, 'utf8'))
  const production = load('production')
  const markdown = `# Benchmark: Rust starter vs Nuxt

Generated by \`node bench/run.mjs\` on ${new Date().toISOString().slice(0, 10)}. Plan and method: [docs/BENCHMARK.md](../docs/BENCHMARK.md).
Machine: ${os.cpus().length} CPU ${os.arch()} ${os.platform()}, Docker ${docker('version', '--format', '{{.Server.Version}}')}.
Production containers are limited to 2 CPUs and 1 GB each. Values are medians with the range in brackets; each metric
ran 3 times unless stated, with the variants alternating.

| Variant | Branch |
|---|---|
${Object.entries(VARIANTS).map(([, variant]) => `| ${variant.label} | \`${variant.branch}\` |`).join('\n')}

${featureSection(load('features'), production)}

${devloopSection(load('devloop'))}

${productionSection(production)}
`
  writeFileSync(`${BENCH_DIR}/REPORT.md`, markdown)
  return `${BENCH_DIR}/REPORT.md`
}
