<script setup lang="ts">
import type { Note } from '#shared/types/notes'

defineProps<{ note: Note }>()
</script>

<template>
  <NuxtLink :to="`/notes/${note.id}`" class="block rounded-xl outline-offset-2 hover:outline-2 hover:outline-border focus-visible:outline-2">
    <Card>
      <CardHeader>
        <CardTitle class="break-words">{{ note.title }}</CardTitle>
        <CardDescription>{{ formatDate(note.createdAt) }}</CardDescription>
      </CardHeader>
      <CardContent v-if="note.body">
        <p class="line-clamp-2 break-words text-muted-foreground">{{ note.body }}</p>
      </CardContent>
    </Card>
  </NuxtLink>
</template>
