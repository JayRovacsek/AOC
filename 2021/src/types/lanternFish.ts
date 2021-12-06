export type LanternFish = {
    spawnTimer: number
}
type LanternFishDay = {spawnTimer: number, population: number}

export const parseFish = (input: string): LanternFish => {
  return {
    spawnTimer: parseInt(input)
  }
}

const populationToSet = (fish: LanternFish[]): LanternFishDay[] => {
  return fish.reduce((accumulator: LanternFishDay[], f) => {
    if (accumulator.some(value => value.spawnTimer === f.spawnTimer)) {
      const match = accumulator.filter(value => value.spawnTimer === f.spawnTimer)[0] ?? { spawnTimer: f.spawnTimer, population: 0 }
      const nonMatch = accumulator.filter(value => value.spawnTimer !== f.spawnTimer)
      return [...nonMatch, { ...match, population: match.population + 1 }]
    }
    return [...accumulator, { ...f, population: 1 }]
  }, [])
}

export const simulateDays = (days: number, fish: LanternFish[]): LanternFishDay[] => {
  const initialSet = populationToSet(fish)
  return Array(days)
    .fill(0)
    .reduce((accumulator) => simulateDay(accumulator), initialSet)
}

const simulateDay = (fish: LanternFishDay[]): LanternFishDay[] => {
  const a = fish.map((f) => {
    if (f.spawnTimer === 0) return [{ spawnTimer: 6, population: f.population }, { spawnTimer: 8, population: f.population }]
    return [{
      spawnTimer: f.spawnTimer - 1,
      population: f.population
    }]
  })
    .flat()
  const r = a
    .reduce((accumulator: LanternFishDay[], set: LanternFishDay, index: number, array: LanternFishDay[]) => {
      const isDuplicate = array.some((x, i) => x.spawnTimer === set.spawnTimer && i > index)
      const merged = accumulator.some((x) => x.spawnTimer === set.spawnTimer)

      if (isDuplicate && merged === false) {
        const matchingDays = array.filter((x, i) => x.spawnTimer === set.spawnTimer && i > index)
        return [...accumulator, { ...set, population: set.population + (matchingDays[0]?.population ?? 0) }]
      }

      if (merged === false) return [...accumulator, set]

      return accumulator
    }, [])

  return r
}
