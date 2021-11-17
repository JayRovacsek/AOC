import { selectPuzzle } from './aoc'
import { dayRange } from './types'
import { argv } from 'process'
import { getInput } from './utils'
import dotenv from 'dotenv'

dotenv.config()

const main = async (): Promise<void> => {
  const days = Array(25).fill(0).map((_, i) => i + 1)
  const day = days.find(d => argv.some((val) => d.toString() === val)) ?? 0

  const input = await getInput(day)

  const puzzle = selectPuzzle(day as dayRange)
  const partOneSolution = puzzle.partOne(input)
  const partTwoSolution = puzzle.partTwo(input)

  console.info(`The solution for part one is: ${await partOneSolution}`)
  console.info(`The solution for part two is: ${await partTwoSolution}`)
}

main()
