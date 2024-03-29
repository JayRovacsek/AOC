/* eslint-disable no-undef */
import { getInput, sumWindows, countWindowIncreases, mostCommonBit, stride, exclusiveStride, garbageStride } from '../utils'

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

  it('should create a stride correctly', async () => {
    const testSet = [
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5], // 3 here
      [0, 1, 2, 3, 4, 5]
    ]

    const subsetA = stride(testSet, 3, 3, 1)
    const subsetB = stride(testSet, 0, 1, 1)
    const subsetC = stride(testSet, 1, 0, 1)

    expect(subsetA).toMatchObject([[2, 3, 4], [2, 3, 4], [2, 3, 4]])
    expect(subsetB).toMatchObject([[0, 1, 2], [0, 1, 2]])
    expect(subsetC).toMatchObject([[0, 1], [0, 1], [0, 1]])
  })

  it('should create an exclusive stride correctly', async () => {
    const testSet = [
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5], // 3 here
      [0, 1, 2, 3, 4, 5]
    ]

    const subsetA = exclusiveStride(testSet, 3, 3, 1)
    const subsetB = exclusiveStride(testSet, 0, 1, 1)
    const subsetC = exclusiveStride(testSet, 1, 0, 1)

    expect(subsetA).toMatchObject([[2, 3, 4], [2, 4], [2, 3, 4]])
    expect(subsetB).toMatchObject([[0, 2], [0, 1, 2]])
    expect(subsetC).toMatchObject([[0, 1], [1], [0, 1]])
  })

  it('should create an garbage stride correctly', async () => {
    const testSet = [
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5],
      [0, 1, 2, 3, 4, 5], // 3 here
      [0, 1, 2, 3, 4, 5]
    ]

    const subsetA = garbageStride(testSet, 3, 3, 1)
    const subsetB = garbageStride(testSet, 0, 1, 1)
    const subsetC = garbageStride(testSet, 1, 0, 1)

    expect(subsetA).toMatchObject([[3], [2, 4], [3]])
    expect(subsetB).toMatchObject([[0, 2], [1]])
    expect(subsetC).toMatchObject([[0], [1], [0]])
  })
})
