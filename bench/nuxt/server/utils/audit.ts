import { auditEvents } from '../db/schema'
import type { Tx } from '../db/client'
import type { Actor } from './actor'

export type AuditAction = 'created' | 'updated' | 'deleted'

export async function recordAudit(tx: Tx, actor: Actor, action: AuditAction, entity: string, entityIds: number[]) {
  if (entityIds.length === 0) {
    return
  }
  await tx.insert(auditEvents).values(entityIds.map(entityId => ({ actorId: actor.id, action, entity, entityId })))
}
