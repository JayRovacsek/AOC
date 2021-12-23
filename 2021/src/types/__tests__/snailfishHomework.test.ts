/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { explodeApplies, Homework, nth, parseHomework, top } from '../snailfishHomework'

const homework: Homework = {
  0: [4],
  1: [3],
  2: [2],
  3: [1],
  4: [9, 8]
}

const nonExplodableHomework: Homework = {
  0: [4],
  1: [3],
  2: [2],
  3: [1]
}

const testInput = `[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]`

describe('Homework', () => {
  it('identify explodable instances correctly', async () => {
    const happyPath = explodeApplies(homework)
    expect(happyPath).toEqual(true)

    const unhappyPath = explodeApplies(nonExplodableHomework)
    expect(unhappyPath).toEqual(false)
  })

  it('should identify the top element of a homework object correctly', async () => {
    const val = top(homework)
    expect(val).toMatchObject({ index: 4, value: [9, 8] })

    const altVal = top(nonExplodableHomework)
    expect(altVal).toMatchObject({ index: 3, value: [1] })
  })

  it('should identify the top element of a homework object correctly', async () => {
    const val = top(homework)
    expect(val).toMatchObject({ index: 4, value: [9, 8] })

    const altVal = top(nonExplodableHomework)
    expect(altVal).toMatchObject({ index: 3, value: [1] })
  })

  it('should identify the nth element of a homework object correctly', async () => {
    const zero = nth(homework, 0)
    expect(zero).toMatchObject({ index: 0, value: [4] })
    const one = nth(homework, 1)
    expect(one).toMatchObject({ index: 1, value: [3] })
    const two = nth(homework, 2)
    expect(two).toMatchObject({ index: 2, value: [2] })
    const three = nth(homework, 3)
    expect(three).toMatchObject({ index: 3, value: [1] })
    const four = nth(homework, 4)
    expect(four).toMatchObject({ index: 4, value: [9, 8] })
  })

  it('should parse numeric values suitably', async () => {
    const zero = nth(homework, 0)
    expect(zero).toMatchObject({ index: 0, value: [4] })
    const one = nth(homework, 1)
    expect(one).toMatchObject({ index: 1, value: [3] })
    const two = nth(homework, 2)
    expect(two).toMatchObject({ index: 2, value: [2] })
    const three = nth(homework, 3)
    expect(three).toMatchObject({ index: 3, value: [1] })
    const four = nth(homework, 4)
    expect(four).toMatchObject({ index: 4, value: [9, 8] })
  })

  it('should parse homework suitably', async () => {
    const homeworks = testInput.split('\n').map(x => parseHomework(x))
    expect(homeworks[0]).toMatchObject({ 0: [1, 2] })
  })
})
