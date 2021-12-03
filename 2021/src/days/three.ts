import { Day } from '../interfaces/day'
import { parseDiagnostic } from '../types/diagnosticCode'

export const dayThree: Day = {
  day: 3,
  partOne: async (input: string): Promise<string> => {
    const { powerConsumption } = parseDiagnostic(input.split('\n'))
    return powerConsumption.toString()
  },
  partTwo: async (input: string): Promise<string> => {
    const { lifeSupportRating } = parseDiagnostic(input.split('\n'))
    return lifeSupportRating.toString()
  }
}
