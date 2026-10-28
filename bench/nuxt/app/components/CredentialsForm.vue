<script setup lang="ts">
import { LogIn, UserPlus } from 'lucide-vue-next'
import type { CredentialsMode } from '~/composables/useCredentialsForm'

const props = defineProps<{ mode: CredentialsMode }>()
const signingUp = computed(() => props.mode === 'sign-up')
const { email, password, errors, saving, submit } = useCredentialsForm(props.mode)
</script>

<template>
  <div class="mx-auto max-w-sm">
    <PageHeader :title="signingUp ? 'Create account' : 'Sign in'" />
    <form class="flex flex-col gap-5" @submit.prevent="submit">
      <FormField id="email" label="Email" :error="errors.email">
        <Input id="email" v-model="email" name="email" type="email" autocomplete="email" :aria-invalid="Boolean(errors.email)" aria-describedby="email-error" />
      </FormField>
      <FormField id="password" label="Password" :error="errors.password">
        <Input id="password" v-model="password" name="password" type="password" :autocomplete="signingUp ? 'new-password' : 'current-password'" :aria-invalid="Boolean(errors.password)" aria-describedby="password-error" />
      </FormField>
      <p v-if="errors[FORM_ERROR]" class="text-sm text-destructive" role="alert">{{ errors[FORM_ERROR] }}</p>
      <Button type="submit" :disabled="saving">
        <template v-if="signingUp"><UserPlus /> Create account</template>
        <template v-else><LogIn /> Sign in</template>
      </Button>
    </form>
    <p class="mt-6 text-sm text-muted-foreground">
      <template v-if="signingUp">Already have an account? <NuxtLink class="text-foreground underline" to="/sign-in">Sign in</NuxtLink></template>
      <template v-else>No account yet? <NuxtLink class="text-foreground underline" to="/sign-up">Create one</NuxtLink></template>
    </p>
  </div>
</template>
