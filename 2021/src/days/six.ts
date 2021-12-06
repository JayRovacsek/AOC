import { Day } from '../interfaces/day'
import { parseFish, simulateDays } from '../types/lanternFish'

export const six: Day = {
  day: 6,
  partOne: async (input: string): Promise<string> => {
    const fish = input.split(',').filter(x => /[0-8]/.test(x)).map(x => parseFish(x))
    const populationSet = simulateDays(80, fish)
    const numberOfFish = populationSet.reduce((accumulator, set) => set.population + accumulator, 0)
    return `${numberOfFish}`
  },
  partTwo: async (input: string): Promise<string> => {
    const fish = input.split(',').filter(x => /[0-8]/.test(x)).map(x => parseFish(x))
    const populationSet = simulateDays(256, fish)
    const numberOfFish = populationSet.reduce((accumulator, set) => set.population + accumulator, 0)
    return `${numberOfFish}`
  }
}
