<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'

const props = defineProps<{ page: number, pageCount: number, total: number }>()
const links = computed(() => neighbours(props.page, props.pageCount))
</script>

<template>
  <div class="mt-6 flex flex-col items-center gap-3 sm:flex-row sm:justify-between">
    <p class="whitespace-nowrap text-sm text-muted-foreground tabular-nums">
      Page {{ page }} of {{ pageCount }} · {{ formatCount(total) }} notes
    </p>
    <div class="flex gap-2">
      <Button v-if="links[0]" variant="ghost" as-child>
        <NuxtLink :to="{ path: '/notes', query: { page: links[0] } }"><ChevronLeft /> Previous</NuxtLink>
      </Button>
      <Button v-if="links[1]" variant="ghost" as-child>
        <NuxtLink :to="{ path: '/notes', query: { page: links[1] } }">Next <ChevronRight /></NuxtLink>
      </Button>
    </div>
  </div>
</template>
