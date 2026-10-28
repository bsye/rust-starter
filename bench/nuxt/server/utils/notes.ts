import { and, count, desc, eq, isNull, type SQL } from 'drizzle-orm'
import { notes } from '../db/schema'
import type { Db } from '../db/client'
import type { Actor } from './actor'
import { recordAudit } from './audit'
import { invalidInput, notFound } from './errors'
import { noteInput } from '../../shared/utils/validation'
import type { Note, NotePage } from '../../shared/types/notes'

export const NOTES_PAGE_SIZE = 20
const ENTITY = 'note'

type NoteRow = typeof notes.$inferSelect

function toNote(row: NoteRow): Note {
  return {
    id: row.id,
    title: row.title,
    body: row.body,
    createdAt: row.createdAt.toISOString(),
    updatedAt: row.updatedAt.toISOString(),
  }
}

function scoped(actor: Actor, ...conditions: SQL[]) {
  return and(eq(notes.ownerId, actor.id), isNull(notes.deletedAt), ...conditions)
}

async function find(db: Db, actor: Actor, id: number): Promise<NoteRow> {
  const [row] = await db.select().from(notes).where(scoped(actor, eq(notes.id, id)))
  if (!row) {
    throw notFound()
  }
  return row
}

function validated(input: unknown) {
  const parsed = noteInput.safeParse(input)
  if (!parsed.success) {
    throw invalidInput(parsed.error)
  }
  return parsed.data
}

export async function listNotes(db: Db, actor: Actor, requestedPage: number): Promise<NotePage> {
  const page = Math.max(1, Math.trunc(requestedPage) || 1)
  const [rows, [totals]] = await Promise.all([
    db.select().from(notes).where(scoped(actor)).orderBy(desc(notes.id))
      .limit(NOTES_PAGE_SIZE).offset((page - 1) * NOTES_PAGE_SIZE),
    db.select({ total: count() }).from(notes).where(scoped(actor)),
  ])
  const total = totals?.total ?? 0
  return { items: rows.map(toNote), total, page, pageCount: Math.ceil(total / NOTES_PAGE_SIZE) }
}

export async function getNote(db: Db, actor: Actor, id: number): Promise<Note> {
  return toNote(await find(db, actor, id))
}

export async function createNote(db: Db, actor: Actor, input: unknown): Promise<Note> {
  const data = validated(input)
  return db.transaction(async (tx) => {
    const [row] = await tx.insert(notes).values({ ownerId: actor.id, ...data }).returning()
    await recordAudit(tx, actor, 'created', ENTITY, [row!.id])
    return toNote(row!)
  })
}

export async function updateNote(db: Db, actor: Actor, id: number, input: unknown): Promise<Note> {
  const data = validated(input)
  await find(db, actor, id)
  return db.transaction(async (tx) => {
    const [row] = await tx.update(notes).set({ ...data, updatedAt: new Date() })
      .where(scoped(actor, eq(notes.id, id))).returning()
    await recordAudit(tx, actor, 'updated', ENTITY, [id])
    return toNote(row!)
  })
}

export async function deleteNote(db: Db, actor: Actor, id: number): Promise<void> {
  await find(db, actor, id)
  await db.transaction(async (tx) => {
    await tx.update(notes).set({ deletedAt: new Date() }).where(scoped(actor, eq(notes.id, id)))
    await recordAudit(tx, actor, 'deleted', ENTITY, [id])
  })
}

export async function insertNotes(db: Db, actor: Actor, inputs: { title: string, body: string }[]): Promise<void> {
  await db.transaction(async (tx) => {
    const rows = await tx.insert(notes).values(inputs.map(input => ({ ownerId: actor.id, ...input })))
      .returning({ id: notes.id })
    await recordAudit(tx, actor, 'created', ENTITY, rows.map(row => row.id))
  })
}
