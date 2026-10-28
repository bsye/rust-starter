export default defineEventHandler(async (event) => {
  const user = await verifyAccount(useDb(), passwords, await readBody(event))
  await replaceUserSession(event, { user })
  return user
})
