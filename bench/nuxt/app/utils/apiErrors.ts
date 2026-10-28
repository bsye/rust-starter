import type { FieldErrors } from '#shared/utils/validation'

export function toFieldErrors(error: unknown): FieldErrors {
  const data = (error as { data?: { data?: { fields?: FieldErrors } } }).data
  return data?.data?.fields ?? { [FORM_ERROR]: 'Something went wrong. Try again.' }
}
