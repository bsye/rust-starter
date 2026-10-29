import { prodApp } from './parity.mjs'
import { TOOLS_IMAGE, docker, log } from './lib.mjs'

const RUNS = 5
const PAGES = { signIn: '/sign-in', notes: '/notes?page=1' }

function audit(app, path, cookie) {
  const headers = cookie ? ['--extra-headers', JSON.stringify({ Cookie: cookie })] : []
  const script = 'CHROME_PATH="$(node -e "console.log(require(\'playwright\').chromium.executablePath())")" exec lighthouse "$@"'
  const output = docker('run', '--rm', '--network', 'host', '--entrypoint', 'sh', TOOLS_IMAGE, '-c', script, 'lighthouse',
    `${prodApp(app).url}${path}`, '--quiet', '--output=json', '--output-path=stdout', '--only-categories=performance',
    '--chrome-flags=--headless=new --no-sandbox', ...headers)
  const report = JSON.parse(output)
  const requests = report.audits['network-requests'].details.items
  const transferred = type => requests.filter(item => item.resourceType === type).reduce((sum, item) => sum + item.transferSize, 0)
  return {
    score: Math.round(report.categories.performance.score * 100),
    lcpMs: Math.round(report.audits['largest-contentful-paint'].numericValue),
    tbtMs: Math.round(report.audits['total-blocking-time'].numericValue),
    totalBytes: Math.round(report.audits['total-byte-weight'].numericValue),
    scriptBytes: transferred('Script'),
    wasmBytes: requests.filter(item => item.url.endsWith('.wasm')).reduce((sum, item) => sum + item.transferSize, 0),
  }
}

export function lighthouse(app, cookie) {
  return Object.fromEntries(Object.entries(PAGES).map(([name, path]) => {
    const runs = []
    for (let run = 0; run < RUNS; run++) {
      log(`lighthouse ${app} ${path} (${run + 1}/${RUNS})`)
      runs.push(audit(app, path, name === 'notes' ? cookie : undefined))
    }
    return [name, runs]
  }))
}
