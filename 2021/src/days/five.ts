import { Grid } from '../classes/grid'
import { Line, matchesLineFormat } from '../classes/line'
import { Day } from '../interfaces/day'

export const five: Day = {
  day: 5,
  partOne: async (input: string): Promise<string> => {
    const lines = input.split('\n').filter((x) => matchesLineFormat(x)).map((x) => new Line(x)).filter((line) => line.isHorizontal() || line.isVertical())
    const grid = new Grid(lines)
    const atLeastTwoOverlap = grid.coordinates
      .reduce((accumulator, row) => accumulator + row
        .reduce((a, x) => {
          if (x >= 2) return a + 1
          return a
        }, 0), 0)
    return `${atLeastTwoOverlap}`
  },
  partTwo: async (input: string): Promise<string> => {
    const lines = input.split('\n').filter((x) => matchesLineFormat(x)).map((x) => new Line(x))
    const grid = new Grid(lines)
    const atLeastTwoOverlap = grid.coordinates
      .reduce((accumulator, row) => accumulator + row
        .reduce((a, x) => {
          if (x >= 2) return a + 1
          return a
        }, 0), 0)
    return `${atLeastTwoOverlap}`
  }
}
