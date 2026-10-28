<script setup lang="ts">
defineProps<{ label: string, id: string, error?: string }>()
</script>

<template>
  <div class="flex flex-col gap-2">
    <Label :for="id">{{ label }}</Label>
    <slot />
    <p v-if="error" :id="`${id}-error`" class="text-sm text-destructive">{{ error }}</p>
  </div>
</template>
