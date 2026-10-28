import { describe, expect, it } from 'vitest'
import { neighbours } from '../app/utils/pager'
import { isSameOriginWrite } from '../server/utils/origin'

describe('pager', () => {
  it('gives a middle page both neighbours', () => expect(neighbours(3, 5)).toEqual([2, 4]))
  it('gives first and last pages one neighbour', () => {
    expect(neighbours(1, 5)).toEqual([null, 2])
    expect(neighbours(5, 5)).toEqual([4, null])
  })
  it('points a page past the end back to the last page', () => expect(neighbours(9, 5)).toEqual([5, null]))
  it('gives a single page no neighbours', () => expect(neighbours(1, 1)).toEqual([null, null]))
})

describe('origin check', () => {
  const origin = 'https://notes.example.com'
  it('allows writes from our origin', () => expect(isSameOriginWrite('POST', origin, origin)).toBe(true))
  it('blocks writes from another origin or none', () => {
    expect(isSameOriginWrite('POST', 'https://evil.example', origin)).toBe(false)
    expect(isSameOriginWrite('DELETE', undefined, origin)).toBe(false)
  })
  it('allows reads from anywhere', () => expect(isSameOriginWrite('GET', undefined, origin)).toBe(true))
})
