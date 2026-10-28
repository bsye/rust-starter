<script setup lang="ts">
import type { Note } from '#shared/types/notes'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const { data: note, error } = await useFetch<Note>(`/api/notes/${route.params.id}`)
if (error.value) {
  throw createError({ statusCode: error.value.statusCode ?? 500, statusMessage: 'Note not found', fatal: true })
}
</script>

<template>
  <div v-if="note" :key="note.id">
    <PageHeader title="Edit note" />
    <NoteForm :existing="note" />
  </div>
</template>
