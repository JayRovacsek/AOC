import { dayRange } from '../types/dayRange'
import { one } from '../days/one'
import { two } from '../days/two'
import { three } from '../days/three'
import { four } from '../days/four'
import { five } from '../days/five'

export const days = [
  one,
  two,
  three,
  four,
  five
]

export interface Day {
    day: dayRange;
    partOne (input: string): Promise<string>,
    partTwo (input: string): Promise<string>
}
