import { Coordinate, parseCoordinate, defaultCoordinate } from '../types/coordinate'

export const matchesLineFormat = (input: string): boolean => /\d+,\d+ -> \d+,\d+/.test(input)

export class Line {
  start: Coordinate;
  end: Coordinate;
  constructor (input: string) {
    const coordinates = input
      .split('->')
      .map(x => x.trim())
      .map(x => parseCoordinate(x))
    const start = coordinates[0] ?? defaultCoordinate
    const end = coordinates[1] ?? defaultCoordinate
    this.start = start
    this.end = end
  }

  isVertical (): boolean {
    if (this.start.x === this.end.x) return true
    return false
  }

  isHorizontal (): boolean {
    if (this.start.y === this.end.y) return true
    return false
  }

  isDiagonal (): boolean {
    if (Math.abs(this.start.y - this.end.y) === Math.abs(this.start.x - this.end.x)) return true
    return false
  }

  length = () => Math.sqrt(Math.pow(this.end.x - this.start.x, 2) + Math.pow(this.end.y - this.start.y, 2))

  intersectingPoints (): Coordinate[] {
    if (this.isVertical()) {
      const x = this.start.x
      const sortedByY = [this.start, this.end].sort((a, b) => a.y - b.y)
      const lowestValue = sortedByY[0]?.y ?? defaultCoordinate.y
      return Array(this.length() + 1)
        .fill(0)
        .map((_, i) => ({ x, y: i + lowestValue }))
    }

    if (this.isHorizontal()) {
      const y = this.start.y
      const sortedByY = [this.start, this.end].sort((a, b) => a.x - b.x)
      const lowestValue = sortedByY[0]?.x ?? defaultCoordinate.x
      return Array(this.length() + 1)
        .fill(0)
        .map((_, i) => ({ x: i + lowestValue, y }))
    }

    const sortedByX = [this.start, this.end].sort((a, b) => a.x - b.x)
    const sortedByY = [this.start, this.end].sort((a, b) => a.y - b.y)

    const isNegativeGradient = (this.start.y - this.end.y) / (this.start.x - this.end.x) < 0

    const steps = Math.abs(this.start.x - this.end.x) + 1

    const greatestXValue = sortedByX[1]?.x ?? defaultCoordinate.x
    const lowestXValue = sortedByX[0]?.x ?? defaultCoordinate.x
    const lowestYValue = sortedByY[0]?.y ?? defaultCoordinate.y

    if (isNegativeGradient) {
      return Array(steps)
        .fill(0)
        .map((_, i) => ({ x: greatestXValue - i, y: lowestYValue + i }))
    }

    return Array(steps)
      .fill(0)
      .map((_, i) => ({ x: i + lowestXValue, y: lowestYValue + i }))
  }
}
