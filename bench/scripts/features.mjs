import { existsSync, readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { SCC_IMAGE, docker } from './lib.mjs'
import { FEATURES, VARIANTS, checkout } from './variants.mjs'

const CONFIG_FILES = {
  rust: ['Cargo.toml', 'Dioxus.toml', 'Dockerfile', 'compose.yaml', 'clippy.toml', 'tailwind.css', 'scripts/check.sh', '.devcontainer/devcontainer.json', '.cargo/config.toml'],
  nuxt: ['package.json', 'nuxt.config.ts', 'Dockerfile', 'compose.yaml', 'drizzle.config.ts', 'eslint.config.mjs', 'vitest.config.ts', 'components.json'],
}

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

const configLines = (root, kind) => CONFIG_FILES[kind]
  .filter(path => existsSync(join(root, path)))
  .reduce((sum, path) => sum + nonBlank(readFileSync(join(root, path), 'utf8')), 0)

function rustTests(root) {
  let lines = 0
  let count = 0
  for (const file of files(`${root}/src`, '.rs').filter(file => !file.includes('/src/components/'))) {
    const text = readFileSync(file, 'utf8')
    const testPart = file.endsWith('_tests.rs') ? text : text.split('#[cfg(test)]').slice(1).join('')
    lines += nonBlank(testPart)
    count += ['#[test]', '#[tokio::test]', '#[sqlx::test'].reduce((sum, marker) => sum + text.split(marker).length - 1, 0)
  }
  return { lines, count }
}

function cargoDependencies(root) {
  let section = ''
  let direct = 0
  for (const line of readFileSync(`${root}/Cargo.toml`, 'utf8').split('\n')) {
    if (line.startsWith('[')) section = line.trim()
    else if (['[dependencies]', '[dev-dependencies]'].includes(section) && line.includes('=')) direct++
  }
  const total = readFileSync(`${root}/Cargo.lock`, 'utf8').split('\n').filter(line => line === '[[package]]').length - 1
  return { direct, total }
}

function rust(root) {
  const code = scc(root, ['src'], 'components')
  const tests = rustTests(root)
  return {
    codeLines: code.code - tests.lines,
    codeFiles: code.files,
    byLanguage: code.byLanguage,
    testLines: tests.lines,
    testCount: tests.count,
    generatedLines: scc(root, ['src/components', 'assets/dx-components-theme.css']).code,
    configLines: configLines(root, 'rust'),
    dependencies: cargoDependencies(root),
  }
}

function nuxt(root) {
  const code = scc(root, ['app', 'server', 'shared', 'scripts'], 'ui,migrations,assets')
  const testCount = files(`${root}/tests`, '.ts').reduce((sum, file) => sum + readFileSync(file, 'utf8').split('\n')
    .filter(line => line.trimStart().startsWith('it(')).length, 0)
  const manifest = JSON.parse(readFileSync(`${root}/package.json`, 'utf8'))
  const lock = JSON.parse(readFileSync(`${root}/package-lock.json`, 'utf8'))
  return {
    codeLines: code.code,
    codeFiles: code.files,
    byLanguage: code.byLanguage,
    testLines: scc(root, ['tests']).code,
    testCount,
    generatedLines: scc(root, ['app/components/ui', 'server/db/migrations', 'app/assets/css']).code,
    configLines: configLines(root, 'nuxt'),
    dependencies: {
      direct: Object.keys(manifest.dependencies).length + Object.keys(manifest.devDependencies).length,
      total: Object.keys(lock.packages).filter(key => key.startsWith('node_modules/')).length,
    },
  }
}

const KINDS = { rust, nuxt }

export function features() {
  return Object.fromEntries(FEATURES.map(variant => [variant, KINDS[VARIANTS[variant].kind](checkout(variant))]))
}
