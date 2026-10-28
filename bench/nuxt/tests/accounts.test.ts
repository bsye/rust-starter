import { describe, expect, it } from 'vitest'
import { users } from '../server/db/schema'
import { createAccount, EMAIL_TAKEN, verifyAccount, WRONG_CREDENTIALS, type Passwords } from '../server/utils/accounts'
import { withTestDb } from './db'

const passwords: Passwords = {
  hash: async password => `hashed:${password}`,
  verify: async (hash, password) => hash === `hashed:${password}`,
}
const PASSWORD = 'correct horse battery'
const fields = (promise: Promise<unknown>) => promise.then(() => null, (error: { data?: { fields?: unknown } }) => error.data?.fields)

describe('accounts', () => {
  it('creates an account with a normalized email', () => withTestDb(async (db) => {
    const user = await createAccount(db, passwords, { email: ' Ada@Example.com ', password: PASSWORD })
    expect(user.email).toBe('ada@example.com')
  }))

  it('rejects a duplicate email in any case', () => withTestDb(async (db) => {
    await createAccount(db, passwords, { email: 'ada@example.com', password: PASSWORD })
    expect(await fields(createAccount(db, passwords, { email: 'ADA@example.com', password: PASSWORD }))).toEqual({ email: EMAIL_TAKEN })
    expect(await db.select().from(users)).toHaveLength(1)
  }))

  it('signs in with the right password', () => withTestDb(async (db) => {
    const created = await createAccount(db, passwords, { email: 'ada@example.com', password: PASSWORD })
    expect(await verifyAccount(db, passwords, { email: 'ADA@example.com ', password: PASSWORD })).toEqual(created)
  }))

  it('gives a wrong password and an unknown email the same answer', () => withTestDb(async (db) => {
    await createAccount(db, passwords, { email: 'ada@example.com', password: PASSWORD })
    const wrongPassword = await fields(verifyAccount(db, passwords, { email: 'ada@example.com', password: 'wrong horse battery' }))
    const unknownEmail = await fields(verifyAccount(db, passwords, { email: 'bob@example.com', password: PASSWORD }))
    expect(wrongPassword).toEqual({ form: WRONG_CREDENTIALS })
    expect(unknownEmail).toEqual(wrongPassword)
  }))
})
