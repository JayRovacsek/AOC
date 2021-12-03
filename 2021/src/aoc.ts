import { template } from './days/template'
import { Day, days } from './interfaces/day'
import { dayRange } from './types/dayRange'

export const selectPuzzle = (input: dayRange): Day => days.find(d => d.day === input) ?? template

export default selectPuzzle
