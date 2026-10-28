import { readFileSync, writeFileSync } from 'node:fs'
import os from 'node:os'
import { BENCH_DIR, docker } from './lib.mjs'

const median = values => {
  const sorted = [...values].sort((a, b) => a - b)
  const middle = Math.floor(sorted.length / 2)
  return sorted.length % 2 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2
}
const number = new Intl.NumberFormat('en', { maximumFractionDigits: 1 })
const seconds = ms => `${number.format(ms / 1000)} s`
const millis = ms => `${number.format(ms)} ms`
const mib = value => `${number.format(value)} MiB`
const bytes = value => `${number.format(value / 1_000_000)} MB`
const stat = (values, format) => values.length > 1
  ? `${format(median(values))} (${format(Math.min(...values))} to ${format(Math.max(...values))}, n=${values.length})`
  : format(values[0])
const ratio = (rust, nuxt, higherIsBetter) => {
  const factor = higherIsBetter ? rust / nuxt : nuxt / rust
  return factor >= 1 ? `Rust ${number.format(factor)}x better` : `Nuxt ${number.format(1 / factor)}x better`
}
const row = (label, rust, nuxt, format, higherIsBetter = false) =>
  `| ${label} | ${stat(rust, format)} | ${stat(nuxt, format)} | ${ratio(median(rust), median(nuxt), higherIsBetter)} |`
const table = (rows, label = 'Metric') => [`| ${label} | Rust starter | Nuxt baseline | Difference |`, '|---|---|---|---|', ...rows].join('\n')

function featureSection({ rust, nuxt }, production) {
  const parityRows = Object.keys(production.rust.parity).map(item =>
    `| ${item} | ${production.rust.parity[item] ? 'pass' : 'FAIL'} | ${production.nuxt.parity[item] ? 'pass' : 'FAIL'} |`)
  return `## Features

${table([
  row('Hand-written code (lines)', [rust.codeLines], [nuxt.codeLines], number.format),
  row('Test code (lines)', [rust.testLines], [nuxt.testLines], number.format),
  row('Tests', [rust.testCount], [nuxt.testCount], number.format, true),
  row('Generated code (lines)', [rust.generatedLines], [nuxt.generatedLines], number.format),
  row('Config (lines)', [rust.configLines], [nuxt.configLines], number.format),
  row('Direct dependencies', [rust.dependencies.direct], [nuxt.dependencies.direct], number.format),
  row('All dependencies (lockfile)', [rust.dependencies.total], [nuxt.dependencies.total], number.format),
])}

Code is counted with [scc](https://github.com/boyter/scc). Hand-written code excludes tests, generated stock components,
generated migrations and theme CSS. Rust: ${Object.entries(rust.byLanguage).map(([name, lines]) => `${name} ${lines}`).join(', ')} lines including inline tests.
Nuxt: ${Object.entries(nuxt.byLanguage).map(([name, lines]) => `${name} ${lines}`).join(', ')}.

### Parity checklist

| Check | Rust starter | Nuxt baseline |
|---|---|---|
${parityRows.join('\n')}`
}

function devloopSection({ rust, nuxt }) {
  const markup = app => app.markupChange.map(sample => sample.ms)
  const reloads = app => app.markupChange.filter(sample => sample.reloaded).length
  return `## Developer loop

${table([
  row('First run (clone to first page, cold caches)', rust.firstRunMs, nuxt.firstRunMs, seconds),
  row('Restart (warm caches)', rust.restartMs, nuxt.restartMs, seconds),
  row('Markup change visible in the browser', markup(rust), markup(nuxt), millis),
  row('Server code change served', rust.logicChangeMs, nuxt.logicChangeMs, seconds),
  row('Lint, typecheck and tests (warm)', rust.checksMs, nuxt.checksMs, seconds),
  row('Production image build, cold', rust.imageBuildMs.cold, nuxt.imageBuildMs.cold, seconds),
  row('Production image build, after a one-line change', rust.imageBuildMs.warm, nuxt.imageBuildMs.warm, seconds),
])}

Full page reloads during markup changes: Rust ${reloads(rust)} of ${rust.markupChange.length}, Nuxt ${reloads(nuxt)} of ${nuxt.markupChange.length}.`
}

function productionSection({ rust, nuxt }) {
  const paths = Object.keys(rust.load)
  const load = (app, path, mode, key) => app.load[path].map(run => run[mode][key])
  const throughputRows = paths.map(path => row(`\`${path}\` requests/s`, load(rust, path, 'throughput', 'rps'), load(nuxt, path, 'throughput', 'rps'), number.format, true))
  const latencyRows = paths.flatMap(path => [
    row(`\`${path}\` p50 at 200 req/s`, load(rust, path, 'fixedRate', 'p50'), load(nuxt, path, 'fixedRate', 'p50'), millis),
    row(`\`${path}\` p99 at 200 req/s`, load(rust, path, 'fixedRate', 'p99'), load(nuxt, path, 'fixedRate', 'p99'), millis),
  ])
  const peak = app => paths.flatMap(path => app.load[path].map(run => run.peakMemoryMiB))
  const lighthouse = (app, page, key) => app.lighthouse[page].map(run => run[key])
  const lighthouseRows = ['signIn', 'notes'].flatMap(page => [
    row(`${page} score`, lighthouse(rust, page, 'score'), lighthouse(nuxt, page, 'score'), number.format, true),
    row(`${page} LCP`, lighthouse(rust, page, 'lcpMs'), lighthouse(nuxt, page, 'lcpMs'), millis),
    row(`${page} total blocking time`, lighthouse(rust, page, 'tbtMs'), lighthouse(nuxt, page, 'tbtMs'), millis),
    row(`${page} bytes transferred`, lighthouse(rust, page, 'totalBytes'), lighthouse(nuxt, page, 'totalBytes'), bytes),
  ])
  return `## Production

${table([
  row('Image size', [rust.imageBytes], [nuxt.imageBytes], bytes),
  row('Cold start to healthy (incl. migrations)', rust.coldStartMs, nuxt.coldStartMs, millis),
  row('Memory, idle', [rust.idleMemoryMiB], [nuxt.idleMemoryMiB], mib),
  row('Memory, peak under load', peak(rust), peak(nuxt), mib),
  row('Sign-in p50 (password hashing)', rust.signIn.map(run => run.p50), nuxt.signIn.map(run => run.p50), millis),
])}

### Throughput (50 connections, ${rust.load[paths[0]].length} runs of 20 s after a 5 s warm-up)

${table(throughputRows, 'Endpoint')}

### Latency at a fixed 200 requests/s (latency corrected)

${table(latencyRows, 'Endpoint')}

### Lighthouse (mobile preset, 5 runs)

${table(lighthouseRows, 'Page')}`
}

export function report(resultsDir) {
  const load = name => JSON.parse(readFileSync(`${resultsDir}/${name}.json`, 'utf8'))
  const production = load('production')
  const markdown = `# Benchmark: Rust starter vs Nuxt

Generated by \`node bench/run.mjs\` on ${new Date().toISOString().slice(0, 10)}. Plan and method: [docs/BENCHMARK.md](../docs/BENCHMARK.md).
Machine: ${os.cpus().length} CPU ${os.arch()} ${os.platform()}, Docker ${docker('version', '--format', '{{.Server.Version}}')}.
Production containers are limited to 2 CPUs and 1 GB each. Values are medians with the range and run count.

${featureSection(load('features'), production)}

${devloopSection(load('devloop'))}

${productionSection(production)}
`
  writeFileSync(`${BENCH_DIR}/REPORT.md`, markdown)
  return `${BENCH_DIR}/REPORT.md`
}
