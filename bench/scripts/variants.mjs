import { mkdtempSync, rmSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { REPO_DIR, sh } from './lib.mjs'

export const VARIANTS = {
  rust07: { label: 'Rust, Dioxus 0.7', kind: 'rust', branch: 'dioxus-0.7', devPort: 9091, prodPort: 9081, image: 'rust-starter:prod' },
  rust07fast: { label: 'Rust, Dioxus 0.7, mold', kind: 'rust', branch: 'faster-builds', devPort: 9093 },
  rust08: { label: 'Rust, Dioxus 0.8 alpha, mold, hot-patch', kind: 'rust', branch: 'main', devPort: 9094, prodPort: 9083, image: 'rust-starter-08:prod' },
  nuxt: { label: 'Nuxt', kind: 'nuxt', branch: 'main', subdir: 'bench/nuxt', devPort: 9092, prodPort: 9082, image: 'nuxt-starter:prod' },
}

export const FEATURES = ['rust07', 'rust08', 'nuxt']
export const PRODUCTION = ['rust07', 'rust08', 'nuxt']
export const DEVLOOP = (process.env.BENCH_VARIANTS ?? 'rust07,rust07fast,rust08,nuxt').split(',')
export const IMAGE_BUILDS = ['rust07', 'rust07fast', 'nuxt']

const WORK_DIR = mkdtempSync(join(tmpdir(), 'rust-starter-bench-'))
const clones = new Map()

export function checkout(variant) {
  const { branch, subdir = '' } = VARIANTS[variant]
  const key = `${variant}-${branch}`
  if (!clones.has(key)) {
    const dir = join(WORK_DIR, key)
    sh('git', ['clone', '--quiet', '--branch', branch, REPO_DIR, dir])
    clones.set(key, dir)
  }
  return join(clones.get(key), subdir)
}

export function cleanupCheckouts() {
  rmSync(WORK_DIR, { recursive: true, force: true })
}
