import { readFileSync } from 'node:fs'
import { eq } from 'drizzle-orm'
import { describe, expect, it } from 'vitest'
import { auditEvents, notes, users } from '../server/db/schema'
import type { Db } from '../server/db/client'
import { trustedActor, type Actor } from '../server/utils/actor'
import { createNote, deleteNote, getNote, insertNotes, listNotes, NOTES_PAGE_SIZE, updateNote } from '../server/utils/notes'
import { withTestDb } from './db'

const input = (title: string) => ({ title, body: '' })

async function actor(db: Db, email: string): Promise<Actor> {
  const [user] = await db.insert(users).values({ email, passwordHash: 'not a real hash' }).returning()
  return trustedActor(user!.id)
}

async function twoUsers(db: Db) {
  return [await actor(db, 'ada@example.com'), await actor(db, 'bob@example.com')] as const
}

const status = (promise: Promise<unknown>) => promise.then(() => 200, (error: { statusCode?: number }) => error.statusCode)

describe('notes', () => {
  it('only queries notes through scoped()', () => {
    const source = readFileSync('server/utils/notes.ts', 'utf8')
    for (const statement of source.split('.from(notes)').slice(1)) {
      expect(statement.slice(0, 40)).toContain('.where(scoped(actor')
    }
  })

  it('lists newest first with totals', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    for (let n = 1; n <= NOTES_PAGE_SIZE + 1; n++) {
      await createNote(db, ada, input(`Note ${n}`))
    }
    const first = await listNotes(db, ada, 1)
    expect(first.items).toHaveLength(NOTES_PAGE_SIZE)
    expect(first.items[0]!.title).toBe(`Note ${NOTES_PAGE_SIZE + 1}`)
    expect([first.total, first.pageCount]).toEqual([NOTES_PAGE_SIZE + 1, 2])
    expect((await listNotes(db, ada, 2)).items).toHaveLength(1)
  }))

  it('returns an empty page past the end with the real total', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    await createNote(db, ada, input('Only note'))
    const page = await listNotes(db, ada, 9)
    expect(page.items).toEqual([])
    expect([page.total, page.page, page.pageCount]).toEqual([1, 9, 1])
    expect((await listNotes(db, ada, 0)).page).toBe(1)
  }))

  it('never shows or changes another user\'s notes', () => withTestDb(async (db) => {
    const [ada, bob] = await twoUsers(db)
    const note = await createNote(db, ada, input('Ada\'s note'))
    expect((await listNotes(db, bob, 1)).total).toBe(0)
    expect(await status(getNote(db, bob, note.id))).toBe(404)
    expect(await status(updateNote(db, bob, note.id, input('Hijacked')))).toBe(404)
    expect(await status(deleteNote(db, bob, note.id))).toBe(404)
    expect((await getNote(db, ada, note.id)).title).toBe('Ada\'s note')
  }))

  it('saves nothing for invalid input', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    expect(await status(createNote(db, ada, input(' ')))).toBe(422)
    expect((await listNotes(db, ada, 1)).total).toBe(0)
  }))

  it('updates with clean input and moves updatedAt', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    const note = await createNote(db, ada, input('Draft'))
    const updated = await updateNote(db, ada, note.id, input('  Final '))
    expect(updated.title).toBe('Final')
    expect(updated.updatedAt > note.updatedAt).toBe(true)
  }))

  it('hides a deleted note but keeps the row', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    const note = await createNote(db, ada, input('Old'))
    await deleteNote(db, ada, note.id)
    expect(await status(getNote(db, ada, note.id))).toBe(404)
    const [row] = await db.select().from(notes).where(eq(notes.id, note.id))
    expect(row!.deletedAt).not.toBeNull()
  }))

  it('records who changed what', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    const note = await createNote(db, ada, input('Draft'))
    await updateNote(db, ada, note.id, input('Final'))
    await deleteNote(db, ada, note.id)
    const events = await db.select().from(auditEvents).where(eq(auditEvents.entityId, note.id)).orderBy(auditEvents.id)
    expect(events.map(event => event.action)).toEqual(['created', 'updated', 'deleted'])
    expect(events.every(event => event.actorId === ada.id && event.entity === 'note')).toBe(true)
  }))

  it('audits every note inserted in bulk', () => withTestDb(async (db) => {
    const [ada] = await twoUsers(db)
    await insertNotes(db, ada, [input('One'), input('Two')])
    expect((await listNotes(db, ada, 1)).total).toBe(2)
    expect(await db.select().from(auditEvents)).toHaveLength(2)
  }))
})
