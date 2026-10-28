export function neighbours(page: number, pageCount: number): [number | null, number | null] {
  const previous = page > 1 && pageCount > 0 ? Math.min(page - 1, pageCount) : null
  const next = page < pageCount ? page + 1 : null
  return [previous, next]
}
