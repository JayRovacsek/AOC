import { dayRange } from '../types/dayRange'
import { dayOne } from '../days/one'
import { dayTwo } from '../days/two'
import { dayThree } from '../days/three'
import { dayFour } from '../days/four'

export const days = [
  dayOne,
  dayTwo,
  dayThree,
  dayFour
]

export interface Day {
    day: dayRange;
    partOne (input: string): Promise<string>,
    partTwo (input: string): Promise<string>
}
