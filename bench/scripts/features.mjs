import { readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { REPO_DIR, SCC_IMAGE, docker } from './lib.mjs'

const NUXT_DIR = `${REPO_DIR}/bench/nuxt`

const nonBlank = text => text.split('\n').filter(line => line.trim() !== '').length

function scc(root, paths, excludeDir) {
  const args = ['run', '--rm', '-v', `${root}:/src:ro`, '-w', '/src', SCC_IMAGE, 'scc', '--format', 'json']
  if (excludeDir) args.push('--exclude-dir', excludeDir)
  const languages = JSON.parse(docker(...args, ...paths))
  return {
    code: languages.reduce((sum, language) => sum + language.Code, 0),
    files: languages.reduce((sum, language) => sum + language.Count, 0),
    byLanguage: Object.fromEntries(languages.map(language => [language.Name, language.Code])),
  }
}

function files(dir, extension) {
  return readdirSync(dir, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile() && entry.name.endsWith(extension))
    .map(entry => join(entry.parentPath, entry.name))
}

function rustTests() {
  let lines = 0
  let count = 0
  for (const file of files(`${REPO_DIR}/src`, '.rs').filter(file => !file.includes('/src/components/'))) {
    const text = readFileSync(file, 'utf8')
    const testPart = file.endsWith('_tests.rs') ? text : text.split('#[cfg(test)]').slice(1).join('')
    lines += nonBlank(testPart)
    count += ['#[test]', '#[tokio::test]', '#[sqlx::test'].reduce((sum, marker) => sum + text.split(marker).length - 1, 0)
  }
  return { lines, count }
}

function nuxtTests() {
  const testFiles = files(`${NUXT_DIR}/tests`, '.ts')
  const count = testFiles.reduce((sum, file) => sum + readFileSync(file, 'utf8').split('\n')
    .filter(line => line.trimStart().startsWith('it(')).length, 0)
  return { lines: scc(NUXT_DIR, ['tests']).code, count }
}

function cargoDependencies() {
  let section = ''
  let direct = 0
  for (const line of readFileSync(`${REPO_DIR}/Cargo.toml`, 'utf8').split('\n')) {
    if (line.startsWith('[')) section = line.trim()
    else if (['[dependencies]', '[dev-dependencies]'].includes(section) && line.includes('=')) direct++
  }
  const total = readFileSync(`${REPO_DIR}/Cargo.lock`, 'utf8').split('\n').filter(line => line === '[[package]]').length - 1
  return { direct, total }
}

function npmDependencies() {
  const manifest = JSON.parse(readFileSync(`${NUXT_DIR}/package.json`, 'utf8'))
  const lock = JSON.parse(readFileSync(`${NUXT_DIR}/package-lock.json`, 'utf8'))
  return {
    direct: Object.keys(manifest.dependencies).length + Object.keys(manifest.devDependencies).length,
    total: Object.keys(lock.packages).filter(key => key.startsWith('node_modules/')).length,
  }
}

const configLines = (root, paths) => paths.reduce((sum, path) => sum + nonBlank(readFileSync(join(root, path), 'utf8')), 0)

export function features() {
  const rustCode = scc(REPO_DIR, ['src'], 'components')
  const rustTestStats = rustTests()
  const nuxtTestStats = nuxtTests()
  return {
    rust: {
      codeLines: rustCode.code - rustTestStats.lines,
      codeFiles: rustCode.files,
      byLanguage: rustCode.byLanguage,
      testLines: rustTestStats.lines,
      testCount: rustTestStats.count,
      generatedLines: scc(REPO_DIR, ['src/components', 'assets/dx-components-theme.css']).code,
      configLines: configLines(REPO_DIR, ['Cargo.toml', 'Dioxus.toml', 'Dockerfile', 'compose.yaml', 'clippy.toml', 'tailwind.css', 'scripts/check.sh', '.devcontainer/devcontainer.json']),
      dependencies: cargoDependencies(),
    },
    nuxt: {
      ...(() => {
        const code = scc(NUXT_DIR, ['app', 'server', 'shared', 'scripts'], 'ui,migrations,assets')
        return { codeLines: code.code, codeFiles: code.files, byLanguage: code.byLanguage }
      })(),
      testLines: nuxtTestStats.lines,
      testCount: nuxtTestStats.count,
      generatedLines: scc(NUXT_DIR, ['app/components/ui', 'server/db/migrations', 'app/assets/css']).code,
      configLines: configLines(NUXT_DIR, ['package.json', 'nuxt.config.ts', 'Dockerfile', 'compose.yaml', 'drizzle.config.ts', 'eslint.config.mjs', 'vitest.config.ts', 'components.json']),
      dependencies: npmDependencies(),
    },
  }
}
