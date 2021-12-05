import { Line } from './line'

export class Grid {
    coordinates: number[][]
    lines: Line[]
    constructor (lines?: Line[]) {
      const gridSizeX = Math.max(...lines?.map(line => Math.max(line.start.x, line.end.x)) ?? [])
      const gridSizeY = Math.max(...lines?.map(line => Math.max(line.start.y, line.end.y)) ?? [])
      const gridSize = Math.max(gridSizeX, gridSizeY) + 1
      this.coordinates = Array(gridSize).fill(0).map(() => Array(gridSize).fill(0))
      this.lines = lines ?? []
      lines?.forEach((line) => {
        line
          .intersectingPoints()
          .forEach((coordinate) => {
          // @ts-ignore
            this.coordinates[coordinate.x][coordinate.y] = this.coordinates[coordinate.x][coordinate.y] + 1
          })
      })
    }

    pointsWithValueEqualToOrGreater (input: number) {
      return this.coordinates
        .reduce((accumulator, row) => accumulator + row
          .reduce((a, x) => {
            if (x >= input) return a + 1
            return a
          }, 0), 0)
    }
}
