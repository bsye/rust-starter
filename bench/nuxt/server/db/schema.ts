import { bigint, bigserial, index, pgTable, text, timestamp } from 'drizzle-orm/pg-core'

const createdAt = () => timestamp('created_at', { withTimezone: true }).notNull().defaultNow()

export const users = pgTable('users', {
  id: bigserial('id', { mode: 'number' }).primaryKey(),
  email: text('email').notNull().unique(),
  passwordHash: text('password_hash').notNull(),
  createdAt: createdAt(),
})

export const notes = pgTable('notes', {
  id: bigserial('id', { mode: 'number' }).primaryKey(),
  ownerId: bigint('owner_id', { mode: 'number' }).notNull().references(() => users.id),
  title: text('title').notNull(),
  body: text('body').notNull(),
  createdAt: createdAt(),
  updatedAt: timestamp('updated_at', { withTimezone: true }).notNull().defaultNow(),
  deletedAt: timestamp('deleted_at', { withTimezone: true }),
}, table => [index('idx_notes_owner_id').on(table.ownerId, table.id)])

export const auditEvents = pgTable('audit_events', {
  id: bigserial('id', { mode: 'number' }).primaryKey(),
  actorId: bigint('actor_id', { mode: 'number' }).notNull().references(() => users.id),
  action: text('action').notNull(),
  entity: text('entity').notNull(),
  entityId: bigint('entity_id', { mode: 'number' }).notNull(),
  createdAt: createdAt(),
}, table => [index('idx_audit_events_entity').on(table.entity, table.entityId)])
