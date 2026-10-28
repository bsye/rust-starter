<script setup lang="ts">
import { NotebookText, UserPlus } from 'lucide-vue-next'

const { loggedIn } = useUserSession()
</script>

<template>
  <section class="flex flex-col items-start gap-4">
    <h1 class="text-3xl font-semibold tracking-tight">Nuxt starter</h1>
    <Button v-if="loggedIn" as-child>
      <NuxtLink to="/notes"><NotebookText /> Open notes</NuxtLink>
    </Button>
    <Button v-else as-child>
      <NuxtLink to="/sign-up"><UserPlus /> Create account</NuxtLink>
    </Button>
  </section>
</template>
