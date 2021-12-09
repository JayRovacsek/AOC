import { dayRange } from '../types/dayRange'
import { one } from '../days/one'
import { two } from '../days/two'
import { three } from '../days/three'
import { four } from '../days/four'
import { five } from '../days/five'
import { six } from '../days/six'
import { seven } from '../days/seven'
import { eight } from '../days/eight'
import { nine } from '../days/nine'

export const days = [
  one,
  two,
  three,
  four,
  five,
  six,
  seven,
  eight,
  nine
]

export interface Day {
    day: dayRange;
    partOne (input: string): Promise<string>,
    partTwo (input: string): Promise<string>
}
