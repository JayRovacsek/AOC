const numericRegex = /\d+/

type Direction = 'forward' | 'down' | 'up' | 'invalid'

export type Instruction = {
    direction: Direction,
    units: number
}

export const parseInstruction = (input: string): Instruction => {
  switch (true) {
    case input.includes('forward'): {
      return {
        direction: 'forward',
        units: input.match(numericRegex)?.reduce((acc, y) => acc + parseInt(y), 0) ?? 0
      }
    }
    case input.includes('down'): {
      return {
        direction: 'down',
        units: input.match(numericRegex)?.reduce((acc, y) => acc + parseInt(y), 0) ?? 0
      }
    }
    case input.includes('up'): {
      return {
        direction: 'up',
        units: input.match(numericRegex)?.reduce((acc, y) => acc + parseInt(y), 0) ?? 0
      }
    }
    default: {
      return {
        direction: 'invalid',
        units: input.match(numericRegex)?.reduce((acc, y) => acc + parseInt(y), 0) ?? 0
      }
    }
  }
}
