import type { Passwords } from './accounts'

export const passwords: Passwords = {
  hash: password => hashPassword(password),
  verify: (hash, password) => verifyPassword(hash, password),
}
