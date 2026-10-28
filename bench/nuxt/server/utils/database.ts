import { createDb, migrateWithLock, type Db } from '../db/client'

let db: Db | undefined
let ready: Promise<void> | undefined

function required(name: string, value: string): string {
  if (!value) {
    throw new Error(`${name} must be set`)
  }
  return value
}

export function useDb(): Db {
  db ??= createDb(required('NUXT_DATABASE_URL', useRuntimeConfig().databaseUrl))
  return db
}

export function databaseReady(): Promise<void> {
  const config = useRuntimeConfig()
  ready ??= migrateWithLock(required('NUXT_DATABASE_URL', config.databaseUrl), config.migrationsDir)
  return ready
}
