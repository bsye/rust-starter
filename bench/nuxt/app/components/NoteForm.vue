<script setup lang="ts">
import { Save } from 'lucide-vue-next'
import type { Note } from '#shared/types/notes'

const props = defineProps<{ existing?: Note }>()
const { title, body, errors, saving, submit } = useNoteForm(props.existing)
</script>

<template>
  <form class="flex max-w-xl flex-col gap-5" @submit.prevent="submit">
    <FormField id="title" label="Title" :error="errors.title">
      <Input id="title" v-model="title" name="title" :aria-invalid="Boolean(errors.title)" aria-describedby="title-error" />
    </FormField>
    <FormField id="body" label="Body" :error="errors.body">
      <Textarea id="body" v-model="body" name="body" :rows="8" :aria-invalid="Boolean(errors.body)" aria-describedby="body-error" />
    </FormField>
    <p v-if="errors[FORM_ERROR]" class="text-sm text-destructive" role="alert">{{ errors[FORM_ERROR] }}</p>
    <div>
      <Button type="submit" :disabled="saving"><Save /> Save note</Button>
    </div>
  </form>
</template>
