<script setup lang="ts">
definePageMeta({ middleware: 'auth' })
</script>

<template>
  <div>
    <PageHeader title="New note" />
    <NoteForm />
  </div>
</template>
