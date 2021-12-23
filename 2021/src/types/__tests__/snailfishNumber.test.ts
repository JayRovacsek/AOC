/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { addSnailfishNumber, explodeSnailfishNumber, splitSnailfishNumber } from '../snailfishNumber'

const one = 1
const oneArray = [1, 1]
const two = 2
const twoArray = [2, 2]
const onePlusTwo = [1, 2]
const oneArrayPlusTwoArray = [[1, 1], [2, 2]]
const onePlusTwoArray = [1, [2, 2]]

describe('SnailfishNumber', () => {
  it('should add snailfish numbers correctly', async () => {
    const addedNumbers = addSnailfishNumber(one, two)
    expect(addedNumbers).toMatchObject(onePlusTwo)
  })

  it('should add snailfish numbers (array) correctly', async () => {
    const addedNumbers = addSnailfishNumber(oneArray, twoArray)
    expect(addedNumbers).toMatchObject(oneArrayPlusTwoArray)
  })

  it('should add snailfish numbers (mixed) correctly', async () => {
    const addedNumbers = addSnailfishNumber(one, twoArray)
    expect(addedNumbers).toMatchObject(onePlusTwoArray)
  })

  it('should explode a number with no left and right correctly', async () => {
    const exploded = explodeSnailfishNumber(oneArray)
    expect(exploded).toEqual(0)
  })

  it('should explode a number with no left and a right correctly', async () => {
    const exploded = explodeSnailfishNumber(oneArray, undefined, one)
    expect(exploded).toEqual([2])
  })

  it('should explode a number with no right and a left correctly', async () => {
    const exploded = explodeSnailfishNumber(oneArray, one)
    expect(exploded).toEqual([2])
  })

  it('should explode a number both left and right correctly', async () => {
    const exploded = explodeSnailfishNumber(oneArray, one, one)
    expect(exploded).toMatchObject(twoArray)
  })

  it('should split a snailfish number correctly', async () => {
    const tenSplit = splitSnailfishNumber(10)
    expect(tenSplit).toMatchObject([5, 5])

    const elevenSplit = splitSnailfishNumber(11)
    expect(elevenSplit).toMatchObject([5, 6])

    const twelveSplit = splitSnailfishNumber(12)
    expect(twelveSplit).toMatchObject([6, 6])
  })
})
