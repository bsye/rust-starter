export function useSignOut() {
  const { clear } = useUserSession()
  const failure = ref<string | null>(null)
  async function run() {
    try {
      await $fetch('/api/auth/sign-out', { method: 'POST' })
      await clear()
      await navigateTo('/sign-in')
    }
    catch {
      failure.value = 'Something went wrong. Try again.'
    }
  }
  return { failure, run }
}
