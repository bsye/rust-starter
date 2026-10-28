import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { BENCH_DIR, log } from './scripts/lib.mjs'

const PARTS = {
  features: async () => (await import('./scripts/features.mjs')).features(),
  production: async () => (await import('./scripts/production.mjs')).production(),
  devloop: async () => (await import('./scripts/devloop.mjs')).devloop(),
}

const [command = 'all', existing] = process.argv.slice(2)
const resultsDir = existing ?? `${BENCH_DIR}/results/${new Date().toISOString().slice(0, 16).replace(':', '-')}`
mkdirSync(resultsDir, { recursive: true })

const parts = command === 'all' ? Object.keys(PARTS) : command === 'report' ? [] : [command]
for (const part of parts) {
  if (!PARTS[part]) throw new Error(`unknown part ${part}; use all, report, ${Object.keys(PARTS).join(', ')}`)
  log(`running ${part}`)
  const file = `${resultsDir}/${part}.json`
  const previous = existsSync(file) ? JSON.parse(readFileSync(file, 'utf8')) : {}
  writeFileSync(file, JSON.stringify({ ...previous, ...await PARTS[part]() }, null, 2))
}

if (['features', 'production', 'devloop'].every(part => existsSync(`${resultsDir}/${part}.json`))) {
  const { report } = await import('./scripts/report.mjs')
  log(`report written to ${report(resultsDir)}`)
}
else {
  log(`results in ${resultsDir}; run the remaining parts with: node bench/run.mjs <part> ${resultsDir}`)
}
