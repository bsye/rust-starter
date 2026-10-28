import { eq } from 'drizzle-orm'
import { users } from '../db/schema'
import type { Db } from '../db/client'
import { invalid, invalidInput } from './errors'
import { credentials, FORM_ERROR, signUpCredentials } from '../../shared/utils/validation'

export interface Passwords {
  hash: (password: string) => Promise<string>
  verify: (hash: string, password: string) => Promise<boolean>
}

export interface AccountUser {
  id: number
  email: string
}

export const WRONG_CREDENTIALS = 'Email or password is wrong'
export const EMAIL_TAKEN = 'An account with this email already exists'
const DECOY_PASSWORD = 'decoy password for unknown emails'
const UNIQUE_VIOLATION = '23505'

let decoyHash: Promise<string> | undefined

function isUniqueViolation(error: unknown): boolean {
  const cause = (error as { cause?: { code?: string } }).cause ?? error
  return (cause as { code?: string }).code === UNIQUE_VIOLATION
}

export async function createAccount(db: Db, passwords: Passwords, input: unknown): Promise<AccountUser> {
  const parsed = signUpCredentials.safeParse(input)
  if (!parsed.success) {
    throw invalidInput(parsed.error)
  }
  const passwordHash = await passwords.hash(parsed.data.password)
  try {
    const [user] = await db.insert(users).values({ email: parsed.data.email, passwordHash })
      .returning({ id: users.id, email: users.email })
    return user!
  }
  catch (error) {
    if (isUniqueViolation(error)) {
      throw invalid({ email: EMAIL_TAKEN })
    }
    throw error
  }
}

export async function verifyAccount(db: Db, passwords: Passwords, input: unknown): Promise<AccountUser> {
  const parsed = credentials.safeParse(input)
  if (!parsed.success) {
    throw invalidInput(parsed.error)
  }
  const [found] = await db.select().from(users).where(eq(users.email, parsed.data.email))
  decoyHash ??= passwords.hash(DECOY_PASSWORD)
  const passwordOk = await passwords.verify(found?.passwordHash ?? await decoyHash, parsed.data.password)
  if (!found || !passwordOk) {
    throw invalid({ [FORM_ERROR]: WRONG_CREDENTIALS })
  }
  return { id: found.id, email: found.email }
}
