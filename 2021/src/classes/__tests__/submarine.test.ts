/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { Submarine } from '../submarine'
import { parseInstruction } from '../../types/instruction'

const testInstructions = `forward 5
down 5
forward 8
up 3
down 8
forward 2`

describe('Submarine', () => {
  it('should execute V1 instructions correctly', async () => {
    const submarine = new Submarine()
    const instructions = testInstructions.split('\n').map(x => parseInstruction(x))
    submarine.executeInstructionsV1(instructions)

    expect(submarine.horizontal).toEqual(15)
    expect(submarine.depth).toEqual(10)
  })

  it('should execute V2 instructions correctly', async () => {
    const submarine = new Submarine()
    const instructions = testInstructions.split('\n').map(x => parseInstruction(x))
    submarine.executeInstructionsV2(instructions)

    expect(submarine.horizontal).toEqual(15)
    expect(submarine.depth).toEqual(60)
  })
})
