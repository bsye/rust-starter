import type { Note } from '#shared/types/notes'
import type { FieldErrors } from '#shared/utils/validation'

export function useNoteForm(existing?: Note) {
  const title = ref(existing?.title ?? '')
  const body = ref(existing?.body ?? '')
  const errors = ref<FieldErrors>({})
  const saving = ref(false)

  async function submit() {
    saving.value = true
    const parsed = noteInput.safeParse({ title: title.value, body: body.value })
    if (!parsed.success) {
      errors.value = fieldErrors(parsed.error)
      saving.value = false
      return
    }
    try {
      const note = await $fetch<Note>(existing ? `/api/notes/${existing.id}` : '/api/notes', {
        method: existing ? 'PUT' : 'POST',
        body: parsed.data,
      })
      await navigateTo(`/notes/${note.id}`)
    }
    catch (error) {
      errors.value = toFieldErrors(error)
      saving.value = false
    }
  }

  return { title, body, errors, saving, submit }
}
