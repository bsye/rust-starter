import { randomUUID } from 'node:crypto'

export const APPS = {
  rust: {
    url: 'http://localhost:9081',
    origin: 'http://rust:8080',
    cookie: 'session',
    credentials: credentials => ({ credentials }),
    note: input => ({ input }),
    fields: json => json?.data?.Invalid,
  },
  nuxt: {
    url: 'http://localhost:9082',
    origin: 'http://nuxt:8080',
    cookie: 'nuxt-session',
    credentials: credentials => credentials,
    note: input => input,
    fields: json => json?.data?.fields,
  },
}

export function client(app, cookie) {
  return async (method, path, body, { origin = true } = {}) => {
    const headers = { 'content-type': 'application/json' }
    if (origin) headers.origin = app.origin
    if (cookie) headers.cookie = cookie
    const response = await fetch(app.url + path, { method, headers, body: body && JSON.stringify(body), redirect: 'manual' })
    const text = await response.text()
    let json
    try {
      json = JSON.parse(text)
    }
    catch {}
    return { status: response.status, json, text, headers: response.headers }
  }
}

export function sessionCookie(app, response) {
  const header = response.headers.getSetCookie().find(value => value.startsWith(`${app.cookie}=`))
  if (!header) throw new Error(`no ${app.cookie} cookie in response`)
  return { pair: header.split(';')[0], attributes: header.toLowerCase() }
}

export async function signUp(app, email, password) {
  const response = await client(app)('POST', '/api/auth/sign-up', app.credentials({ email, password }))
  if (response.status !== 200) throw new Error(`sign-up failed with ${response.status}: ${response.text.slice(0, 200)}`)
  return sessionCookie(app, response)
}

export async function parity(app) {
  const anonymous = client(app)
  const password = randomUUID()
  const email = `parity-${randomUUID()}@example.com`
  const results = {}
  const check = (name, passed) => { results[name] = Boolean(passed) }

  const page = await anonymous('GET', '/notes')
  check('Signed-out page redirects to sign-in', [302, 303].includes(page.status) && page.headers.get('location')?.endsWith('/sign-in'))
  check('Signed-out API answers 401', (await anonymous('GET', '/api/notes?page=1')).status === 401)
  check('Write without Origin answers 403', (await anonymous('POST', '/api/auth/sign-up', app.credentials({ email, password }), { origin: false })).status === 403)
  const short = await anonymous('POST', '/api/auth/sign-up', app.credentials({ email, password: 'short' }))
  check('Short password is a field error', short.status === 422 && app.fields(short.json)?.password)

  const session = await signUp(app, email, password)
  check('Session cookie is HttpOnly and SameSite=Lax', session.attributes.includes('httponly') && session.attributes.includes('samesite=lax'))
  const duplicate = await anonymous('POST', '/api/auth/sign-up', app.credentials({ email: email.toUpperCase(), password }))
  check('Duplicate email in another case is a field error', duplicate.status === 422 && app.fields(duplicate.json)?.email)
  const wrong = await anonymous('POST', '/api/auth/sign-in', app.credentials({ email, password: 'wrong password!' }))
  const unknown = await anonymous('POST', '/api/auth/sign-in', app.credentials({ email: `x${email}`, password }))
  check('Wrong password and unknown email get the same answer', wrong.status === 422 && JSON.stringify(app.fields(wrong.json)) === JSON.stringify(app.fields(unknown.json)))

  const owner = client(app, session.pair)
  const created = await owner('POST', '/api/notes', app.note({ title: '  Parity note ', body: 'body' }))
  check('Create trims the title', created.status === 200 && created.json?.title === 'Parity note')
  const id = created.json?.id
  const pastEnd = await owner('GET', '/api/notes?page=99')
  check('Page past the end is empty with the real total', pastEnd.json?.items?.length === 0 && pastEnd.json?.total === 1)

  const other = client(app, (await signUp(app, `other-${email}`, password)).pair)
  check('Other user cannot read, edit or delete the note', (await other('GET', `/api/notes/${id}`)).status === 404
    && (await other('PUT', `/api/notes/${id}`, app.note({ title: 'Hijack', body: '' }))).status === 404
    && (await other('DELETE', `/api/notes/${id}`)).status === 404
    && (await other('GET', '/api/notes?page=1')).json?.total === 0)

  const updated = await owner('PUT', `/api/notes/${id}`, app.note({ title: 'Edited', body: '' }))
  check('Update saves the new title', updated.status === 200 && updated.json?.title === 'Edited')
  await owner('DELETE', `/api/notes/${id}`)
  check('Deleted note is gone', (await owner('GET', `/api/notes/${id}`)).status === 404)
  check('Unknown page answers 404', (await owner('GET', '/no/such/page')).status === 404)

  await owner('POST', '/api/auth/sign-out', {})
  check('Old cookie stops working after sign-out', (await owner('GET', '/api/notes?page=1')).status === 401)
  return results
}
