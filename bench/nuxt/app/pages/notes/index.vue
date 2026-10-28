<script setup lang="ts">
import { ArrowLeft, Plus, RotateCw, StickyNote } from 'lucide-vue-next'
import type { NotePage } from '#shared/types/notes'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const page = computed(() => Number(route.query.page) || 1)
const { data, status, error, refresh } = await useFetch<NotePage>('/api/notes', { query: { page } })
</script>

<template>
  <div>
    <PageHeader title="Notes">
      <Button as-child>
        <NuxtLink to="/notes/new"><Plus /> New note</NuxtLink>
      </Button>
    </PageHeader>
    <NoteListSkeleton v-if="status === 'pending'" />
    <EmptyState v-else-if="error" title="Something went wrong. Try again.">
      <Button variant="outline" @click="refresh()"><RotateCw /> Try again</Button>
    </EmptyState>
    <EmptyState v-else-if="data && data.total === 0" title="No notes yet">
      <template #icon><StickyNote /></template>
      <Button variant="outline" as-child>
        <NuxtLink to="/notes/new"><Plus /> New note</NuxtLink>
      </Button>
    </EmptyState>
    <template v-else-if="data">
      <EmptyState v-if="data.items.length === 0" title="No notes on this page">
        <template #icon><StickyNote /></template>
        <Button variant="outline" as-child>
          <NuxtLink to="/notes"><ArrowLeft /> First page</NuxtLink>
        </Button>
      </EmptyState>
      <div class="grid gap-3">
        <NoteCard v-for="note in data.items" :key="note.id" :note="note" />
      </div>
      <NotePager :page="data.page" :page-count="data.pageCount" :total="data.total" />
    </template>
  </div>
</template>
