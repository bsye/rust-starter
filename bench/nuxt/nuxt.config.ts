import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: false },
  modules: ['@nuxt/eslint', 'shadcn-nuxt', 'nuxt-auth-utils'],
  css: ['~/assets/css/tailwind.css'],
  vite: {
    plugins: [tailwindcss()],
  },
  shadcn: {
    prefix: '',
    componentDir: '@/components/ui',
  },
  app: {
    head: { title: 'Nuxt starter' },
  },
  runtimeConfig: {
    databaseUrl: '',
    siteOrigin: '',
    migrationsDir: 'server/db/migrations',
  },
})
