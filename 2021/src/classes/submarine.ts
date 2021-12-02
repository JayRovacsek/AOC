import { Instruction } from '../types/instruction'

export class Submarine {
    horizontal: number;
    depth: number;
    aim: number;

    constructor (horizontal: number = 0, depth: number = 0, aim: number = 0) {
      this.horizontal = horizontal
      this.depth = depth
      this.aim = aim
    }

    executeInstructionsV1 (instructions: Instruction[]): void {
      instructions.forEach((instruction) => {
        switch (instruction.direction) {
          case 'down':
            this.depth = this.depth + instruction.units
            break
          case 'up':
            this.depth = this.depth - instruction.units
            break
          case 'forward':
            this.horizontal = this.horizontal + instruction.units
            break
          case 'invalid':
            break
        }
      })
    }

    executeInstructionsV2 (instructions: Instruction[]): void {
      instructions.forEach((instruction) => {
        switch (instruction.direction) {
          case 'down':
            this.aim = this.aim + instruction.units
            break
          case 'up':
            this.aim = this.aim - instruction.units
            break
          case 'forward':
            this.horizontal = this.horizontal + instruction.units
            this.depth = this.depth + (this.aim * instruction.units)
            break
          case 'invalid':
            break
        }
      })
    }
}
