import { Day } from '../interfaces/day'
import { parseFish, simulateDays } from '../types/lanternFish'

export const six: Day = {
  day: 6,
  partOne: async (input: string): Promise<string> => {
    const fish = input.split(',').filter(x => /[0-8]/.test(x)).map(x => parseFish(x))
    const numberOfFish = simulateDays(80, fish)
    return `${numberOfFish}`
  },
  partTwo: async (input: string): Promise<string> => {
    const fish = input.split(',').filter(x => /[0-8]/.test(x)).map(x => parseFish(x))
    const numberOfFish = simulateDays(256, fish)
    return `${numberOfFish}`
  }
}
