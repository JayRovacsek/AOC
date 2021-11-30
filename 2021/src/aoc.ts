import { dayOne } from './days/one'
import { template } from './days/template'
import { Day } from './interfaces'
import { dayRange } from './types'

const days = [
  dayOne
]

export const selectPuzzle = (input: dayRange): Day => days.find(d => d.day === input) ?? template

export default selectPuzzle
