/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { LanternFish, parseFish, simulateDays } from '../lanternFish'

const testInput = '3,4,3,1,2'

describe('Lanternfish', () => {
  it('should parse fish correctly', async () => {
    const fish = testInput.split(',').filter(x => /[0-8]/.test(x)).map(x => parseFish(x))
    expect(fish[0]).toMatchObject({ spawnTimer: 3 })
    expect(fish).toMatchObject([{ spawnTimer: 3 }, { spawnTimer: 4 }, { spawnTimer: 3 }, { spawnTimer: 1 }, { spawnTimer: 2 }])
  })

  it('should emulate fish breeding correctly', async () => {
    const fish = testInput.split(',').filter(x => /[0-8]/.test(x)).map(x => parseFish(x))
    const futureFish: LanternFish[] = simulateDays(80, fish)
    expect(futureFish.length).toEqual(5934)
  })
})
