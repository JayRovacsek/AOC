import { Day } from '../interfaces/day'
import { parseLavaMap } from '../types/lavaMap'

export const nine: Day = {
  day: 9,
  partOne: async (input: string): Promise<string> => {
    const lavaMap = parseLavaMap(input)
    // not 452
    return `${lavaMap.riskSum}`
  },
  partTwo: async (input: string): Promise<string> => {
    return input
  }
}
