export default defineNitroPlugin(() => {
  databaseReady().catch((error) => {
    console.error('database migration failed', error)
    process.exit(1)
  })
})
