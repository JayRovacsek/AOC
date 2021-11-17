import { dayRange } from './types'

export interface Day {
    day: dayRange;
    partOne (input: string): Promise<string>,
    partTwo (input: string): Promise<string>
}
