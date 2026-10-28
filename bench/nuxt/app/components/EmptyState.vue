<script setup lang="ts">
defineProps<{ title: string }>()
</script>

<template>
  <Card>
    <div class="flex flex-col items-center gap-4 px-6 py-12 text-center">
      <div class="text-muted-foreground [&_svg]:size-8"><slot name="icon" /></div>
      <p class="text-lg font-medium">{{ title }}</p>
      <slot />
    </div>
  </Card>
</template>
