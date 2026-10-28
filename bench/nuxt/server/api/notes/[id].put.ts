export default defineEventHandler(async (event) => {
  const actor = await requireActor(event)
  return updateNote(useDb(), actor, Number(getRouterParam(event, 'id')), await readBody(event))
})
