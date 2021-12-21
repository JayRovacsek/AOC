/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { Cave } from '../cave'

const testInstructions = `start-A
start-b
A-c
A-b
b-d
A-end
b-end`

const alternateTestInstructions = `dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc`

const thirdTestInstructions = `fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW`

describe('Cave', () => {
  it('should parse a cave correctly', async () => {
    const cave = new Cave(testInstructions)

    expect(cave.nodes
      .filter(x => x.t1.isStart || x.t2.isEnd).length
    ).toEqual(4)
  })

  it('should parse paths through a cave correctly', async () => {
    const cave = new Cave(testInstructions)
    expect(cave.pathstrings.length).toEqual(10)

    const altCave = new Cave(alternateTestInstructions)
    expect(altCave.pathstrings.length).toEqual(19)

    const thirdCave = new Cave(thirdTestInstructions)
    expect(thirdCave.pathstrings.length).toEqual(226)
  })

  it('should parse paths with a double visit correctly', async () => {
    const cave = new Cave(testInstructions, true)
    expect(cave.pathstrings.length).toEqual(36)

    const altCave = new Cave(alternateTestInstructions, true)
    expect(altCave.pathstrings.length).toEqual(103)

    const thirdCave = new Cave(thirdTestInstructions, true)
    expect(thirdCave.pathstrings.length).toEqual(3509)
  })
})
