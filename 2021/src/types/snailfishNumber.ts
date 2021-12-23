export type SnailfishNumber = SnailfishNumber[] | number

export const addSnailfishNumber = (left: SnailfishNumber, right: SnailfishNumber): SnailfishNumber => {
  return [left, right]
}

export const explodeSnailfishNumber = (input: number[], left?: number, right?: number): SnailfishNumber => {
  if (left === undefined && right === undefined) return 0

  if (left !== undefined && right === undefined) return [left + (input[0] ?? 0)]
  if (left === undefined && right !== undefined) return [right + (input[1] ?? 0)]
  return [(left ?? 0) + (input[0] ?? 0), (right ?? 0) + (input[1] ?? 0)]
}

export const splitSnailfishNumber = (input: number): SnailfishNumber => {
  return [Math.floor(input / 2), Math.ceil(input / 2)]
}
