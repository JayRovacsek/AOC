import { Cave } from '../classes/cave'
import { Day } from '../interfaces/day'

export const twelve: Day = {
  day: 12,
  partOne: async (input: string): Promise<string> => {
    const cave = new Cave(input)
    return `${cave.pathstrings.length}`
  },
  partTwo: async (input: string): Promise<string> => {
    return input
  }
}
