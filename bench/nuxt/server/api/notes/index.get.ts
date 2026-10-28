export default defineEventHandler(async (event) => {
  const actor = await requireActor(event)
  return listNotes(useDb(), actor, Number(getQuery(event).page))
})
