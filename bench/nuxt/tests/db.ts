import postgres from 'postgres'
import { createDb, migrateWithLock, type Db } from '../server/db/client'

export async function withTestDb(run: (db: Db) => Promise<void>): Promise<void> {
  const admin = process.env.NUXT_DATABASE_URL
  if (!admin) {
    throw new Error('NUXT_DATABASE_URL must be set')
  }
  const name = `test_${crypto.randomUUID().replaceAll('-', '')}`
  const sql = postgres(admin, { max: 1, onnotice: () => {} })
  await sql.unsafe(`create database ${name}`)
  const url = new URL(admin)
  url.pathname = `/${name}`
  try {
    await migrateWithLock(url.toString(), 'server/db/migrations')
    const db = createDb(url.toString())
    try {
      await run(db)
    }
    finally {
      await db.$client.end()
    }
  }
  finally {
    await sql.unsafe(`drop database ${name} with (force)`)
    await sql.end()
  }
}
