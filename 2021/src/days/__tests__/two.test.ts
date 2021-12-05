/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { two } from '../two'

const testInput = `forward 5
down 5
forward 8
up 3
down 8
forward 2`

describe('Utils', () => {
  it('should solve part one', async () => {
    const results = await two.partOne(testInput)
    expect(results).toEqual('150')
  })

  it('should solve part two', async () => {
    const results = await two.partTwo(testInput)
    expect(results).toEqual('900')
  })
})
