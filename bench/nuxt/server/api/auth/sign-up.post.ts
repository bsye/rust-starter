export default defineEventHandler(async (event) => {
  const user = await createAccount(useDb(), passwords, await readBody(event))
  await replaceUserSession(event, { user })
  return user
})
