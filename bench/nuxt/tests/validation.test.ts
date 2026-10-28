import { describe, expect, it } from 'vitest'
import { credentials, fieldErrors, noteInput, signUpCredentials, TITLE_MAX_CHARS, BODY_MAX_CHARS, PASSWORD_MAX_CHARS } from '../shared/utils/validation'

function errorFor(schema: typeof noteInput | typeof signUpCredentials, input: unknown, field: string) {
  const parsed = schema.safeParse(input)
  if (parsed.success) {
    throw new Error('expected invalid input')
  }
  return fieldErrors(parsed.error)[field]
}

describe('note input', () => {
  it('trims the title and trailing body whitespace', () => {
    expect(noteInput.parse({ title: '  Groceries  ', body: '  milk\n\n' })).toEqual({ title: 'Groceries', body: '  milk' })
  })
  it('rejects a blank title', () => {
    expect(errorFor(noteInput, { title: '   ', body: '' }, 'title')).toBe('Enter a title')
  })
  it('rejects a title over the limit', () => {
    expect(errorFor(noteInput, { title: 'a'.repeat(TITLE_MAX_CHARS + 1), body: '' }, 'title')).toBe('Use 200 characters or fewer')
  })
  it('counts characters, not code units', () => {
    expect(noteInput.safeParse({ title: '😀'.repeat(TITLE_MAX_CHARS), body: '' }).success).toBe(true)
  })
  it('rejects a body over the limit', () => {
    expect(errorFor(noteInput, { title: 'Title', body: 'a'.repeat(BODY_MAX_CHARS + 1) }, 'body')).toBe('Use 10000 characters or fewer')
  })
})

describe('credentials', () => {
  it('normalizes the email but not the password', () => {
    expect(credentials.parse({ email: '  Ada@Example.COM ', password: ' Secret ' })).toEqual({ email: 'ada@example.com', password: ' Secret ' })
  })
  it('accepts plain addresses', () => {
    for (const email of ['ada@example.com', 'a.b+tag@mail.example.co.uk']) {
      expect(signUpCredentials.safeParse({ email, password: 'correct horse battery' }).success, email).toBe(true)
    }
  })
  it('rejects malformed addresses', () => {
    for (const email of ['', 'ada', '@example.com', 'ada@', 'ada@example', 'ada@@example.com', 'a da@example.com', 'ada@example..com']) {
      expect(signUpCredentials.safeParse({ email, password: 'correct horse battery' }).success, email).toBe(false)
    }
  })
  it('requires a long enough password', () => {
    expect(errorFor(signUpCredentials, { email: 'ada@example.com', password: 'short' }, 'password')).toBe('Use at least 12 characters')
  })
  it('caps password length', () => {
    expect(signUpCredentials.safeParse({ email: 'ada@example.com', password: 'a'.repeat(PASSWORD_MAX_CHARS + 1) }).success).toBe(false)
  })
})
