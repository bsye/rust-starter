export default defineEventHandler(async (event) => {
  const actor = await requireActor(event)
  return createNote(useDb(), actor, await readBody(event))
})
