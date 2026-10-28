import { createError } from 'h3'
import type { ZodError } from 'zod'
import { fieldErrors, type FieldErrors } from '../../shared/utils/validation'

export function invalid(fields: FieldErrors) {
  return createError({ statusCode: 422, statusMessage: 'Check the highlighted fields', data: { fields } })
}

export function invalidInput(error: ZodError) {
  return invalid(fieldErrors(error))
}

export function notFound() {
  return createError({ statusCode: 404, statusMessage: 'Not found' })
}
