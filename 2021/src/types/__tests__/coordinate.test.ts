/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { parseCoordinate } from '../coordinate'

const testInput = `0,9
8,0
9,4`

describe('Utils', () => {
  it('should parse coordinates correctly', async () => {
    const coordinates = testInput.split('\n').map(x => parseCoordinate(x))
    expect(coordinates[0]).toMatchObject({ x: 0, y: 9 })
    expect(coordinates[1]).toMatchObject({ x: 8, y: 0 })
    expect(coordinates[2]).toMatchObject({ x: 9, y: 4 })
  })
})
