<script setup lang="ts">
import { Trash2 } from 'lucide-vue-next'

const props = defineProps<{ id: number }>()
const { open, failure, confirm } = useDeleteNote(props.id)
</script>

<template>
  <AlertDialog v-model:open="open">
    <AlertDialogTrigger as-child>
      <Button variant="destructive" :title="failure ?? ''"><Trash2 /> Delete</Button>
    </AlertDialogTrigger>
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Delete this note?</AlertDialogTitle>
        <AlertDialogDescription>It will disappear from your notes.</AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel>Cancel</AlertDialogCancel>
        <AlertDialogAction @click="confirm">Delete</AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
</template>
