const READ_METHODS = new Set(['GET', 'HEAD', 'OPTIONS'])

export function isSameOriginWrite(method: string, origin: string | undefined, publicOrigin: string): boolean {
  return READ_METHODS.has(method) || origin === publicOrigin
}
