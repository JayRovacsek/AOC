export class Bingo {
  tiles: (number | null)[][];
  completionRound: number

  constructor (input: string[]) {
    this.tiles = input
      .map(x => x.split(/\s+/)
        .map(y => parseInt(y))
        .filter(y => Number.isNaN(y) === false)
      )
      .filter(x => x.length !== 0)
    this.completionRound = 0
  }

  setCompletionRound (input: number) {
    this.completionRound = input
  }

  isComplete () {
    const verticalComplete = this.tiles.some((_, i, t) => t.every(v => v[i] === null))
    const horizontalComplete = this.tiles.some((t) => t.every(v => v === null))
    return verticalComplete || horizontalComplete
  }

  sumOfTiles () {
    return this.tiles.reduce((accumulator, t) =>
      accumulator + t
        .reduce((a: number, x) => a + (x ?? 0), 0)
    , 0)
  }

  applyCall (input: number) {
    this.tiles = this.tiles
      .map(t => t.map(v => v === input ? null : v))
  }
}

export const parseBingoCalls = (input: string): number[] => {
  return input.split(',').map(x => parseInt(x))
}
