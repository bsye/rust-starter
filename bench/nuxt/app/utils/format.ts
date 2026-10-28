const dateFormat = new Intl.DateTimeFormat('en-GB', { timeZone: 'UTC', day: 'numeric', month: 'short', year: 'numeric' })
const countFormat = new Intl.NumberFormat('en')

export const formatDate = (iso: string) => dateFormat.format(new Date(iso))
export const formatCount = (value: number) => countFormat.format(value)
