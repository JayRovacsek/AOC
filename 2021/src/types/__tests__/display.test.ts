/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { parseDisplayV1, parseDisplayV2, Segment } from '../display'

const testInput = `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`

describe('Displays', () => {
  it('should identify 1, 4, 7 and 8 correctly', async () => {
    const segments = testInput
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

    expect(validDisplays.length).toEqual(26)
  })

  it('should parse display outputs correctly', async () => {
    const segments = testInput
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

    const sum = displays.reduce((accumulator, display) => accumulator + display.value, 0)

    expect(sum).toEqual(61229)
  })
})
