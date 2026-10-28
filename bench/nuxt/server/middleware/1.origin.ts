import { isSameOriginWrite } from '../utils/origin'

export default defineEventHandler((event) => {
  const allowed = isSameOriginWrite(event.method, getRequestHeader(event, 'origin'), useRuntimeConfig().siteOrigin)
  if (!allowed) {
    throw createError({ statusCode: 403 })
  }
})
