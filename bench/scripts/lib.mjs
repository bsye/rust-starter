import { execFile, execFileSync } from 'node:child_process'
import { promisify } from 'node:util'
import { performance } from 'node:perf_hooks'
import { fileURLToPath } from 'node:url'

export const BENCH_DIR = fileURLToPath(new URL('..', import.meta.url))
export const REPO_DIR = fileURLToPath(new URL('../..', import.meta.url))
export const TOOLS_IMAGE = 'bench-tools:1'
export const OHA_IMAGE = 'ghcr.io/hatoo/oha:1.16.0'
export const SCC_IMAGE = 'ghcr.io/boyter/scc:v4.1.0'
export const RUNS = Number(process.env.BENCH_RUNS ?? 3)

export function sh(command, args, options = {}) {
  return execFileSync(command, args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], maxBuffer: 64 * 1024 * 1024, ...options }).trim()
}

export const docker = (...args) => sh('docker', args)
export const dockerAsync = async (...args) => (await promisify(execFile)('docker', args, { maxBuffer: 64 * 1024 * 1024 })).stdout.trim()
export const now = () => performance.now()
export const log = message => console.log(`[bench ${new Date().toISOString().slice(11, 19)}] ${message}`)
export const sleep = ms => new Promise(resolve => setTimeout(resolve, ms))

export async function timed(action) {
  const start = now()
  await action()
  return Math.round(now() - start)
}

export async function waitForHttp(url, { timeoutMs = 900_000, body, exact = false } = {}) {
  const deadline = now() + timeoutMs
  while (now() < deadline) {
    try {
      const response = await fetch(url, { redirect: 'manual' })
      const text = await response.text()
      const matches = body === undefined || (exact ? text === body : text.includes(body))
      if (response.status === 200 && matches) {
        return
      }
    }
    catch {}
    await sleep(50)
  }
  throw new Error(`timed out waiting for ${url}${body ? ` to contain ${body}` : ''}`)
}

export function psql(database, sql) {
  return docker('compose', '-f', `${BENCH_DIR}/compose.yaml`, 'exec', '-T', 'db', 'psql', '-U', 'postgres', '-d', database, '-tAc', sql)
}

export async function memoryMiB(container) {
  const usage = (await dockerAsync('stats', '--no-stream', '--format', '{{.MemUsage}}', container)).split('/')[0].trim()
  const value = Number.parseFloat(usage)
  const unit = usage.replace(String(value), '').trim()
  const factor = { KiB: 1 / 1024, MiB: 1, GiB: 1024, B: 1 / (1024 * 1024) }[unit]
  if (factor === undefined) {
    throw new Error(`unexpected memory unit in ${usage}`)
  }
  return Math.round(value * factor * 10) / 10
}
