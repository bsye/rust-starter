import { z } from 'zod'

export const TITLE_MAX_CHARS = 200
export const BODY_MAX_CHARS = 10_000
export const PASSWORD_MIN_CHARS = 12
export const PASSWORD_MAX_CHARS = 128
export const FORM_ERROR = 'form'

export type FieldErrors = Record<string, string>

const chars = (value: string) => [...value].length

export const noteInput = z.object({
  title: z.string().trim()
    .min(1, 'Enter a title')
    .refine(value => chars(value) <= TITLE_MAX_CHARS, `Use ${TITLE_MAX_CHARS} characters or fewer`),
  body: z.string().overwrite(value => value.trimEnd())
    .refine(value => chars(value) <= BODY_MAX_CHARS, `Use ${BODY_MAX_CHARS} characters or fewer`),
})

export type NoteInput = z.infer<typeof noteInput>

const email = z.string().trim().toLowerCase()

export const credentials = z.object({ email, password: z.string() })

export const signUpCredentials = z.object({
  email: email.pipe(z.email('Enter an email address like name@example.com')),
  password: z.string()
    .refine(value => chars(value) >= PASSWORD_MIN_CHARS, `Use at least ${PASSWORD_MIN_CHARS} characters`)
    .refine(value => chars(value) <= PASSWORD_MAX_CHARS, `Use ${PASSWORD_MAX_CHARS} characters or fewer`),
})

export type Credentials = z.infer<typeof credentials>

export function fieldErrors(error: z.ZodError): FieldErrors {
  const errors: FieldErrors = {}
  for (const issue of error.issues) {
    const field = String(issue.path[0] ?? FORM_ERROR)
    errors[field] ??= issue.message
  }
  return errors
}
