/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { parseInstruction } from '../instruction'

const testInstructions = `forward 5
down 5
forward 8
up 3
down 8
forward 2`

describe('Utils', () => {
  it('should parse instructions correctly', async () => {
    const instructions = testInstructions.split('\n').map(x => parseInstruction(x))
    expect(instructions).toMatchObject([
      {
        direction: 'forward',
        units: 5
      },
      {
        direction: 'down',
        units: 5
      },
      {
        direction: 'forward',
        units: 8
      },
      {
        direction: 'up',
        units: 3
      },
      {
        direction: 'down',
        units: 8
      },
      {
        direction: 'forward',
        units: 2
      }
    ])
  })
})
