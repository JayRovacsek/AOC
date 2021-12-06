export type LanternFish = {
    spawnTimer: number
}

export const parseFish = (input: string): LanternFish => {
  return {
    spawnTimer: parseInt(input)
  }
}

export const simulateDays = (days: number, fish: LanternFish[]): LanternFish[] => {
  return Array(days)
    .fill(0)
    .reduce((accumulator) => simulateDay(accumulator), fish)
}

export const simulateDay = (fish: LanternFish[]): LanternFish[] => {
  return fish.map(f => {
    if (f.spawnTimer === 0) return [{ spawnTimer: 8 }, { spawnTimer: 6 }]
    return {
      spawnTimer: f.spawnTimer - 1
    }
  }).flat()
}
