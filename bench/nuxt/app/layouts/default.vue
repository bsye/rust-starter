<script setup lang="ts">
import { LogIn, LogOut } from 'lucide-vue-next'

const { loggedIn, user } = useUserSession()
const route = useRoute()
const signOut = useSignOut()
const inNotes = computed(() => route.path.startsWith('/notes'))
const onAuthPage = computed(() => ['/sign-in', '/sign-up'].includes(route.path))
</script>

<template>
  <div>
    <header class="border-b border-border">
      <nav class="mx-auto flex h-14 max-w-5xl items-center gap-6 px-4">
        <NuxtLink class="font-semibold" to="/">Nuxt starter</NuxtLink>
        <NuxtLink
          v-if="loggedIn"
          to="/notes"
          class="transition-colors hover:text-foreground"
          :class="inNotes ? 'text-foreground' : 'text-muted-foreground'"
        >
          Notes
        </NuxtLink>
        <div class="ml-auto flex items-center gap-3">
          <template v-if="loggedIn">
            <span class="hidden max-w-48 truncate text-sm text-muted-foreground sm:inline" :title="user?.email">{{ user?.email }}</span>
            <Button variant="ghost" :title="signOut.failure.value ?? ''" @click="signOut.run">
              <LogOut />
              Sign out
            </Button>
          </template>
          <Button v-else-if="!onAuthPage" variant="ghost" as-child>
            <NuxtLink to="/sign-in"><LogIn /> Sign in</NuxtLink>
          </Button>
        </div>
      </nav>
    </header>
    <main class="mx-auto max-w-5xl px-4 py-8">
      <slot />
    </main>
  </div>
</template>
