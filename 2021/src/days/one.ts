import { Day } from '../interfaces/day'
import { sumWindows, countWindowIncreases } from '../utils'
export const one: Day = {
  day: 1,

  partOne: async (input: string): Promise<string> => {
    const values = input.split('\n').map(value => parseInt(value)).filter(x => Number.isNaN(x) === false)
    return countWindowIncreases(values).toString()
  },

  partTwo: async (input: string): Promise<string> => {
    const values = input.split('\n').map(value => parseInt(value)).filter(x => Number.isNaN(x) === false)
    const summedWindows = sumWindows(values, 3)
    const result = countWindowIncreases(summedWindows)
    return result.toString()
  }
}
