import { readFileSync, writeFileSync } from 'node:fs'
import { chromium } from 'playwright'

const WAIT_MS = 600_000
const { url, file, cookie, ready, steps } = JSON.parse(process.argv[2])
const original = readFileSync(file, 'utf8')
const visible = (page, text) => page.waitForFunction(value => document.body.innerText.includes(value), text, { timeout: WAIT_MS, polling: 10 })

const browser = await chromium.launch()
const context = await browser.newContext()
if (cookie) {
  const [name, ...value] = cookie.split('=')
  await context.addCookies([{ name, value: value.join('='), url }])
}
const page = await context.newPage()
const hotReloadSocket = page.waitForEvent('websocket', { timeout: WAIT_MS })
await page.goto(url)
await hotReloadSocket
await page.waitForLoadState('networkidle')
await visible(page, ready)
await page.evaluate(() => { window.__benchMarker = true })

const samples = []
try {
  for (const { from, to, expect } of steps) {
    const current = readFileSync(file, 'utf8')
    if (!current.includes(from)) throw new Error(`${file} does not contain ${from}`)
    const start = performance.now()
    writeFileSync(file, current.replace(from, to))
    await visible(page, expect)
    samples.push({ ms: Math.round(performance.now() - start), reloaded: !(await page.evaluate(() => window.__benchMarker === true)) })
    await page.evaluate(() => { window.__benchMarker = true })
    await new Promise(resolve => setTimeout(resolve, 1000))
  }
}
finally {
  writeFileSync(file, original)
}
await visible(page, ready)
await browser.close()
console.log(JSON.stringify(samples))
