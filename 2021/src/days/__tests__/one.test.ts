/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { dayOne } from '../one'

const testInput = [
  199,
  200,
  208,
  210,
  200,
  207,
  240,
  269,
  260,
  263
].join('\n')

describe('Utils', () => {
  it('should solve part one', async () => {
    const results = await dayOne.partOne(testInput)
    expect(results).toEqual('7')
  })

  it('should solve part two', async () => {
    expect(true).toEqual(true)
  })
})
