/* eslint-disable no-undef */
import { getInput, sumWindows, countWindowIncreases, mostCommonBit } from '../utils'

const dayOneTestSet = [
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
]

describe('Utils', () => {
  it('should throw if we do not have a session cookie', async () => {
    process.env.SESSION_COOKIE = undefined
    expect(async () => await getInput(1)).rejects.toThrow('Missing session cookie from env variables.')
  })

  it('should sum windows correctly', async () => {
    const expectedOutput = [
      607, 618, 618, 617, 647, 716, 769, 792
    ]

    const results = sumWindows(dayOneTestSet, 3)

    expect(results).toMatchObject(expectedOutput)
  })

  it('should identify increases correctly', async () => {
    const results = countWindowIncreases(dayOneTestSet)
    expect(results).toEqual(7)
  })

  it('should identify most common bits correctly', async () => {
    const zero = mostCommonBit([1, 0, 0])
    const one = mostCommonBit([1, 1, 0])
    expect(zero).toEqual(0)
    expect(one).toEqual(1)
  })
})
