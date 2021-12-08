import { Day } from '../interfaces/day'
import { Segment, parseDisplayV1, parseDisplayV2 } from '../types/display'

export const eight: Day = {
  day: 8,
  partOne: async (input: string): Promise<string> => {
    const segments = input
      .split('\n')
      .map(x => x.split(' | ')
        .filter((_, i) => i > 0))
      .flat()
      .map(x => x.split(' '))
      .flat()
      .filter(x => /[a-g]/.test(x))
      .map(x => x.split('') as Segment[])

    const displays = segments
      .map(x => parseDisplayV1(x))

    const validDisplays = displays.filter(x => x !== null)

    return `${validDisplays.length}`
  },

  partTwo: async (input: string): Promise<string> => {
    const segments = input
      .split('\n')
      .map(x => {
        const parts = x.replace(' | ', ' ').split(' ')
        if ((parts.length ?? []) > 0) {
          return {
            input: parts.filter((_, i) => i <= 9).map(x => x.split('').sort((a, b) => a < b ? -1 : 1).join('')) as string[],
            output: parts.filter((_, i) => i > 9).map(x => x.split('').sort((a, b) => a < b ? -1 : 1).join('')) as string[]
          }
        }
        return { input: [], output: [] }
      })

    const displays = segments
      .map(x => parseDisplayV2(x))
      .filter(x => x.segments.length > 0)

    const sum = displays.reduce((accumulator, display) => accumulator + display.value, 0)

    return `${sum}`
  }
}
