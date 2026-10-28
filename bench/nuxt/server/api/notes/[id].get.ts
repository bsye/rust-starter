export default defineEventHandler(async (event) => {
  const actor = await requireActor(event)
  return getNote(useDb(), actor, Number(getRouterParam(event, 'id')))
})
