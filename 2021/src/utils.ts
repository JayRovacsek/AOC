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

export const mostCommonBit = (input: Bit[]): Bit => {
  const zeroCount = input.filter(x => x === 0).length
  const oneCount = input.filter(x => x === 1).length
  if (zeroCount > oneCount) return 0
  return 1
}
