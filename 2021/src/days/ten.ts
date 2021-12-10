import { Day } from '../interfaces/day'
import { parseNavigationSubsystem } from '../types/navSubsystem'

export const ten: Day = {
  day: 10,
  partOne: async (input: string): Promise<string> => {
    const subsystem = parseNavigationSubsystem(input)
    return `${subsystem.syntaxErrorScore}`
  },
  partTwo: async (input: string): Promise<string> => {
    const subsystem = parseNavigationSubsystem(input)
    return `${subsystem.completionScore}`
  }
}
