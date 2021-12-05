export type Coordinate = {
    x: number,
    y: number
}

export const defaultCoordinate: Coordinate = {
  x: 0,
  y: 0
}

export const parseCoordinate = (input: string): Coordinate => {
  const parts = input.split(',')
  const x = parseInt(parts[0] ?? '0')
  const y = parseInt(parts[1] ?? '0')
  return {
    x, y
  }
}
