<template>
  <div class="grid gap-3">
    <Card v-for="index in 4" :key="index">
      <CardHeader>
        <Skeleton class="h-5 w-48" />
        <Skeleton class="h-4 w-24" />
      </CardHeader>
      <CardContent><Skeleton class="h-4 w-full" /></CardContent>
    </Card>
  </div>
</template>
