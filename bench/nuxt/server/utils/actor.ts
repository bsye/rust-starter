import type { H3Event } from 'h3'

declare const actorBrand: unique symbol

export type Actor = { readonly id: number, readonly [actorBrand]: true }

export function trustedActor(id: number): Actor {
  return { id } as Actor
}

export async function requireActor(event: H3Event): Promise<Actor> {
  const { user } = await requireUserSession(event)
  return trustedActor(user.id)
}
