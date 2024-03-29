import { Day } from '../interfaces/day'
import { parseCrabSubmarine } from '../types/crabSubmarine'
import { triangleNumber } from '../utils'

export const seven: Day = {
  day: 7,
  partOne: async (input: string): Promise<string> => {
    const crabs = input
      .split(',')
      .filter(x => /\d+/.test(x))
      .map(x => parseCrabSubmarine(x))

    const currentPositions = Array.from(new Set(crabs
      .map(crab => crab.position)))
    const minPosition = Math.min(...currentPositions)
    const maxPosition = Math.max(...currentPositions)

    const possiblePositions = Array((maxPosition - minPosition) + 1).fill(0).map((_, i) => i + minPosition)

    const fuelUtilisation = Math.min(...possiblePositions
      .map((position) => crabs
        .reduce((accumulator, crab) =>
          accumulator + (Math.max(crab.position, position) - Math.min(crab.position, position)), 0))
    )

    return `${fuelUtilisation}`
  },
  partTwo: async (input: string): Promise<string> => {
    const crabs = input
      .split(',')
      .filter(x => /\d+/.test(x))
      .map(x => parseCrabSubmarine(x))

    const currentPositions = Array.from(new Set(crabs.map(crab => crab.position)))
    const minPosition = Math.min(...currentPositions)
    const maxPosition = Math.max(...currentPositions)

    const possiblePositions = Array((maxPosition - minPosition) + 1)
      .fill(0)
      .map((_, i) => i + minPosition)

    const fuelUtilisation = Math.min(...possiblePositions
      .map((position) => crabs
        .reduce((accumulator, crab) =>
          accumulator +
          (triangleNumber(
            Math.max(crab.position, position) - Math.min(crab.position, position)
          ))
        , 0))
    )
    return `${fuelUtilisation}`
  }
}
