import postgres from 'postgres'
import { expect, it } from 'vitest'
import { migrateWithLock } from '../server/db/client'

it('lets two migrations started together both succeed', async () => {
  const admin = process.env.NUXT_DATABASE_URL!
  const name = `test_${crypto.randomUUID().replaceAll('-', '')}`
  const sql = postgres(admin, { max: 1, onnotice: () => {} })
  await sql.unsafe(`create database ${name}`)
  const url = new URL(admin)
  url.pathname = `/${name}`
  try {
    const results = await Promise.allSettled([
      migrateWithLock(url.toString(), 'server/db/migrations'),
      migrateWithLock(url.toString(), 'server/db/migrations'),
    ])
    expect(results.map(result => result.status)).toEqual(['fulfilled', 'fulfilled'])
  }
  finally {
    await sql.unsafe(`drop database ${name} with (force)`)
    await sql.end()
  }
})
