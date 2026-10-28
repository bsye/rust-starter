<script setup lang="ts">
import { ArrowLeft, Pencil } from 'lucide-vue-next'
import type { Note } from '#shared/types/notes'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const { data: note, error } = await useFetch<Note>(`/api/notes/${route.params.id}`)
if (error.value) {
  throw createError({ statusCode: error.value.statusCode ?? 500, statusMessage: 'Note not found', fatal: true })
}
</script>

<template>
  <div v-if="note">
    <NuxtLink to="/notes" class="mb-3 inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground">
      <ArrowLeft class="size-4" /> All notes
    </NuxtLink>
    <PageHeader :title="note.title">
      <Button as-child>
        <NuxtLink :to="`/notes/${note.id}/edit`"><Pencil /> Edit</NuxtLink>
      </Button>
      <DeleteNoteButton :id="note.id" />
    </PageHeader>
    <p class="mb-4 text-sm text-muted-foreground">{{ formatDate(note.createdAt) }}</p>
    <Card v-if="note.body">
      <div class="whitespace-pre-wrap break-words px-6">{{ note.body }}</div>
    </Card>
  </div>
</template>
