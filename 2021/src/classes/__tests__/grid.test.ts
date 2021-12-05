/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { Grid } from '../grid'
import { Line, matchesLineFormat } from '../line'

const testInput = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`

describe('Utils', () => {
  it('should build a grid correctly', async () => {
    const smallLine = '0,1 -> 1,0'
    const lines = [smallLine].map(x => new Line(x))
    const grid = new Grid(lines)
    expect(grid.coordinates).toMatchObject([[0, 1], [1, 0]])
  })

  it('should populate coordinates with correct values', async () => {
    const lines = testInput
      .split('\n')
      .filter(x => matchesLineFormat(x))
      .map(x => new Line(x))
      .filter(line => line.isHorizontal() || line.isVertical())
    const grid = new Grid(lines)
    const overlapTotal = grid.coordinates
      .reduce((accumulator, x) => accumulator + x
        .reduce((a, y) => a + y, 0)
      , 0)

    expect(overlapTotal).toEqual(26)
  })

  it('should identify points with a select value or greater where lines are only vertical or horizontal', async () => {
    const lines = testInput
      .split('\n')
      .filter(x => matchesLineFormat(x))
      .map(x => new Line(x))
      .filter(line => line.isHorizontal() || line.isVertical())
    const grid = new Grid(lines)
    expect(grid.pointsWithValueEqualToOrGreater(2)).toEqual(5)
  })

  it('should identify points with a select value or greater', async () => {
    const lines = testInput
      .split('\n')
      .filter(x => matchesLineFormat(x))
      .map(x => new Line(x))
    const grid = new Grid(lines)
    expect(grid.pointsWithValueEqualToOrGreater(2)).toEqual(12)
  })
})
