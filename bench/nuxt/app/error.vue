<script setup lang="ts">
import type { NuxtError } from '#app'
import { ArrowLeft, FileQuestion } from 'lucide-vue-next'

const props = defineProps<{ error: NuxtError }>()
const title = computed(() => (props.error.statusCode === 404 ? 'Page not found' : 'Something went wrong. Try again.'))
</script>

<template>
  <NuxtLayout>
    <EmptyState :title="title">
      <template #icon><FileQuestion /></template>
      <Button variant="outline" @click="clearError({ redirect: '/' })">
        <ArrowLeft />
        Go home
      </Button>
    </EmptyState>
  </NuxtLayout>
</template>
