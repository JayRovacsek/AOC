import axios from 'axios'
import { Bit } from './types/bit'

export const getInput = async (day: number): Promise<string> => {
  if (process.env.SESSION_COOKIE === 'undefined') throw new Error('Missing session cookie from env variables.')
  try {
    const request = await axios.get(`https://adventofcode.com/2021/day/${day}/input`, {
      headers: {
        Cookie: `session=${process.env.SESSION_COOKIE} `,
        Accept: 'text/plain'
      }
    })

    return request.data
  } catch (error) {
    if (typeof error === 'string') {
      console.error(error)
      throw new Error(error)
    }

    console.error(JSON.stringify(error, null, 4))
    throw new Error(JSON.stringify(error, null, 4))
  }
}

export const countWindowIncreases = (values: number[]): number => {
  return values.reduce((accumulator, value, index, array) => {
    if (index === array.length) return accumulator
    // @ts-ignore
    if (array[index + 1] !== undefined && value < array[index + 1]) {
      return accumulator + 1
    }
    return accumulator
  }, 0)
}

export const sumWindows = (values: number[], windowSize: number): number[] => {
  const results = values
    .map((_, index, array) => {
      const vals = array.filter((_, i) => i >= index && i < (index + windowSize))

      if (vals.length === windowSize) return vals.reduce((a, v) => a + v, 0)
      return null
    })
    .filter(x => x !== null)

  // @ts-ignore
  return results
}

export const mostCommonBit = (input: Bit[], priorityBit?: Bit): Bit => {
  const zeroCount = input.filter(x => x === 0).length
  const oneCount = input.filter(x => x === 1).length
  if (priorityBit !== undefined && zeroCount === oneCount) return priorityBit
  if (zeroCount > oneCount) return 0
  return 1
}

export const triangleNumber = (input: number): number => {
  return ((input * (input + 1)) / 2)
}

export const stride = <Type>(input: Type[][], targetRow: number, targetColumn: number, strideSize: number): Type[][] => {
  const rowLimit = Math.max(...input.map(x => x.length))
  const columnLimit = input[0]?.length ?? 1

  const minimumRow = targetRow - strideSize > 0 ? targetRow - strideSize : 0
  const maximumRow = targetRow + strideSize < rowLimit ? targetRow + strideSize : rowLimit
  const minimumColumn = targetColumn - strideSize > 0 ? targetColumn - strideSize : 0
  const maximumColumn = targetColumn + strideSize < columnLimit ? targetColumn + strideSize : columnLimit

  return input
    .filter((_, i) => i <= maximumRow && i >= minimumRow)
    .map((row) => row.filter((_, j) => j <= maximumColumn && j >= minimumColumn))
}

export const exclusiveStride = <Type>(input: Type[][], targetRow: number, targetColumn: number, strideSize: number): Type[][] => {
  const rowLimit = Math.max(...input.map(x => x.length))
  const columnLimit = input[0]?.length ?? 1

  const minimumRow = targetRow - strideSize > 0 ? targetRow - strideSize : 0
  const maximumRow = targetRow + strideSize < rowLimit ? targetRow + strideSize : rowLimit
  const minimumColumn = targetColumn - strideSize > 0 ? targetColumn - strideSize : 0
  const maximumColumn = targetColumn + strideSize < columnLimit ? targetColumn + strideSize : columnLimit

  return input
    .map((row, i) => row.filter((_, j) => {
      if (i === targetRow) {
        return j <= maximumColumn &&
        j >= minimumColumn &&
        i <= maximumRow &&
        i >= minimumRow &&
        j !== targetColumn
      }

      return j <= maximumColumn &&
        j >= minimumColumn &&
        i <= maximumRow &&
        i >= minimumRow
    }
    )).filter(x => x.length !== 0)
}

export const garbageStride = <Type>(input: Type[][], targetRow: number, targetColumn: number, strideSize: number): Type[][] => {
  const rowLimit = Math.max(...input.map(x => x.length))
  const columnLimit = input[0]?.length ?? 1

  const minimumRow = targetRow - strideSize > 0 ? targetRow - strideSize : 0
  const maximumRow = targetRow + strideSize < rowLimit ? targetRow + strideSize : rowLimit
  const minimumColumn = targetColumn - strideSize > 0 ? targetColumn - strideSize : 0
  const maximumColumn = targetColumn + strideSize < columnLimit ? targetColumn + strideSize : columnLimit

  return input.map((row, i) => row.filter((_, j) => {
    if (i === targetRow && j === targetColumn) return false
    if (i === targetRow) return j >= minimumColumn && j <= maximumColumn
    if (j === targetColumn) return i >= minimumRow && i <= maximumRow
    return false
  }))
    .filter(x => x.length !== 0)
}
