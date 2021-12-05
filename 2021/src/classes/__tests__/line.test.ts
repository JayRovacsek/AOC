/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { Line } from '../line'

const testInput = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`

describe('Utils', () => {
  it('should parse lines correctly', async () => {
    const lines = testInput.split('\n').map(x => new Line(x))
    expect(lines[0]).toMatchObject({ start: { x: 0, y: 9 }, end: { x: 5, y: 9 } })
    expect(lines[1]).toMatchObject({ start: { x: 8, y: 0 }, end: { x: 0, y: 8 } })
    expect(lines[2]).toMatchObject({ start: { x: 9, y: 4 }, end: { x: 3, y: 4 } })
  })

  it('should identify vertical or horizontal lines correctly', async () => {
    const lines = testInput.split('\n').map(x => new Line(x))
    const filteredLines = lines.filter(line => line.isHorizontal() || line.isVertical())
    expect(filteredLines.length).toEqual(6)
  })

  it('should identify diagonal lines correctly', async () => {
    const lines = testInput.split('\n').map(x => new Line(x))
    const filteredLines = lines.filter(line => line.isDiagonal())
    expect(filteredLines.length).toEqual(4)
  })

  it('should calculate the length of a line correctly', async () => {
    const lines = testInput.split('\n').map(x => new Line(x))
    const length = lines[0]?.length()
    expect(length).toEqual(5)
  })

  it('should identify intersecting coordinates of a line correctly', async () => {
    const lines = testInput.split('\n').map(x => new Line(x))
    const points = lines[0]?.intersectingPoints()
    expect(points?.length).toEqual(6)
    expect(points).toMatchObject([
      {
        x: 0, y: 9
      },
      {
        x: 1, y: 9
      },
      {
        x: 2, y: 9
      },
      {
        x: 3, y: 9
      },
      {
        x: 4, y: 9
      },
      {
        x: 5, y: 9
      }
    ])
  })
})
