/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { parseLavaMap } from '../lavaMap'

const testInput = `2199943210
3987894921
9856789892
8767896789
9899965678`

describe('LavaMap', () => {
  it('should correctly parse a map', async () => {
    const map = parseLavaMap(testInput)

    expect(map.map.length).toEqual(5)
    expect(map.map[0]).toMatchObject([2, 1, 9, 9, 9, 4, 3, 2, 1, 0])
    expect(map.lowPoints).toMatchObject([
      {
        row: 0,
        column: 1
      },
      {
        row: 0,
        column: 9
      },
      {
        row: 2,
        column: 2
      },
      {
        row: 4,
        column: 6
      }
    ])

    expect(map.riskSum).toEqual(15)
  })
})
