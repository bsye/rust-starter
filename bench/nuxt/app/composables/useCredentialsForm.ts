import type { FieldErrors } from '#shared/utils/validation'

export type CredentialsMode = 'sign-in' | 'sign-up'

export function useCredentialsForm(mode: CredentialsMode) {
  const email = ref('')
  const password = ref('')
  const errors = ref<FieldErrors>({})
  const saving = ref(false)
  const { fetch: refreshSession } = useUserSession()

  async function submit() {
    saving.value = true
    const input = { email: email.value, password: password.value }
    if (mode === 'sign-up') {
      const parsed = signUpCredentials.safeParse(input)
      if (!parsed.success) {
        errors.value = fieldErrors(parsed.error)
        saving.value = false
        return
      }
    }
    try {
      await $fetch(`/api/auth/${mode}`, { method: 'POST', body: input })
      await refreshSession()
      await navigateTo('/notes')
    }
    catch (error) {
      errors.value = toFieldErrors(error)
      saving.value = false
    }
  }

  return { email, password, errors, saving, submit }
}
