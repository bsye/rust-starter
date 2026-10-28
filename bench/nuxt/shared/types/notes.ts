export interface Note {
  id: number
  title: string
  body: string
  createdAt: string
  updatedAt: string
}

export interface NotePage {
  items: Note[]
  total: number
  page: number
  pageCount: number
}
