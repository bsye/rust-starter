import { readFileSync, writeFileSync } from 'node:fs'
import { chromium } from 'playwright'

const [url, file, label, runsText] = process.argv.slice(2)
const runs = Number(runsText)
const original = readFileSync(file, 'utf8')
if (!original.includes(label)) {
  throw new Error(`${file} does not contain ${label}`)
}

const browser = await chromium.launch()
const page = await browser.newPage()
await page.goto(url)
await page.waitForFunction(text => document.body.innerText.includes(text), label, { timeout: 600_000 })
await page.evaluate(() => { window.__benchMarker = true })

const samples = []
let current = label
try {
  for (let run = 1; run <= runs; run++) {
    const next = `${label} ${run}`
    const start = performance.now()
    writeFileSync(file, readFileSync(file, 'utf8').replace(current, next))
    await page.waitForFunction(text => document.body.innerText.includes(text), next, { timeout: 600_000, polling: 10 })
    samples.push({ ms: Math.round(performance.now() - start), reloaded: !(await page.evaluate(() => window.__benchMarker === true)) })
    current = next
    await page.evaluate(() => { window.__benchMarker = true })
    await new Promise(resolve => setTimeout(resolve, 1000))
  }
}
finally {
  writeFileSync(file, original)
  await browser.close()
}
console.log(JSON.stringify(samples))
