import { dayRange } from '../types'
import { dayOne } from '../days/one'
import { dayTwo } from '../days/two'

export const days = [
  dayOne,
  dayTwo
]

export interface Day {
    day: dayRange;
    partOne (input: string): Promise<string>,
    partTwo (input: string): Promise<string>
}
