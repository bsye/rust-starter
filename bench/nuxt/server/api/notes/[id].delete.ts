export default defineEventHandler(async (event) => {
  const actor = await requireActor(event)
  await deleteNote(useDb(), actor, Number(getRouterParam(event, 'id')))
  return { ok: true }
})
