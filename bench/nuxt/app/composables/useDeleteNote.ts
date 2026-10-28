export function useDeleteNote(id: number) {
  const open = ref(false)
  const failure = ref<string | null>(null)
  const deleting = ref(false)

  async function confirm() {
    if (deleting.value) {
      return
    }
    deleting.value = true
    try {
      await $fetch(`/api/notes/${id}`, { method: 'DELETE' })
      await navigateTo('/notes')
    }
    catch {
      failure.value = 'Something went wrong. Try again.'
      deleting.value = false
      open.value = false
    }
  }

  return { open, failure, deleting, confirm }
}
