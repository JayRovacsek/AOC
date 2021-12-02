import { Day } from '../interfaces/day'
import { parseInstruction } from '../types/instruction'
import { Submarine } from '../classes/submarine'

export const dayTwo: Day = {
  day: 2,

  partOne: async (input: string): Promise<string> => {
    const instructions = input.split('\n').map(value => parseInstruction(value))
    const submarine = new Submarine()
    submarine.executeInstructionsV1(instructions)
    return `${submarine.horizontal * submarine.depth}`
  },

  partTwo: async (input: string): Promise<string> => {
    const instructions = input.split('\n').map(value => parseInstruction(value))
    const submarine = new Submarine()
    submarine.executeInstructionsV2(instructions)
    return `${submarine.horizontal * submarine.depth}`
  }
}
