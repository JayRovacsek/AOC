import { stride } from '../utils'

export type LavaMap = {
    map: number[][],
    lowPoints: { column: number, row: number }[],
    riskSum: number
}

export const parseLavaMap = (input: string): LavaMap => {
  const rows = input.split('\n').filter(x => {
    return /\d+/.test(x)
  })

  const map = rows.map(row => row.split('').map(x => parseInt(x)))

  const lowPoints = map.map((row, i) => row.map((_, j) => {
    // @ts-ignore
    const currentValue = map[i][j] ?? 0
    const set = stride(map, i, j, 1)
    const results = set.map((row) => {
      if (row.every(x => currentValue <= x)) {
        return true
      }
      return false
    })

    if (results.every(v => v === true)) {
      return { row: i, column: j }
    }
    return null
  })
  ).flat()
    .filter(x => x !== null)

  const riskSum = lowPoints.reduce((accumulator, lowPoint) => {
    // @ts-ignore
    const value = map[lowPoint?.row ?? 0][lowPoint?.column ?? 0] ?? 0
    return accumulator + value + 1
  }, 0)

  return {
    map,
    // @ts-ignore
    lowPoints,
    riskSum
  }
}
