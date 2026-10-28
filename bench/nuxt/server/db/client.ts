import { drizzle, type PostgresJsDatabase } from 'drizzle-orm/postgres-js'
import { migrate } from 'drizzle-orm/postgres-js/migrator'
import postgres from 'postgres'
import * as schema from './schema'

export type Db = PostgresJsDatabase<typeof schema>
export type Tx = Parameters<Parameters<Db['transaction']>[0]>[0]

const MAX_CONNECTIONS = 10
const MIGRATION_LOCK_KEY = 7_401_220_002

export function createDb(url: string): Db {
  return drizzle(postgres(url, { max: MAX_CONNECTIONS }), { schema })
}

export async function migrateWithLock(url: string, migrationsFolder: string): Promise<void> {
  const client = postgres(url, { max: 1, onnotice: () => {} })
  try {
    await client`select pg_advisory_lock(${MIGRATION_LOCK_KEY})`
    await migrate(drizzle(client), { migrationsFolder })
  }
  finally {
    await client.end()
  }
}
