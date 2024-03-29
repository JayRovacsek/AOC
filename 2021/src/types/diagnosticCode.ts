import { mostCommonBit } from '../utils'
import { Bit } from './bit'

export type Diagnostic = {
    raw: string[],
    gammaRate: string,
    epsilonRate: string,
    oxygenRating: string,
    co2ScrubberRating: string,
    powerConsumption: number,
    lifeSupportRating: number
}

export const parseMatchingCode = (bitPredicate: 'mcb' | 'lcb', codes: string[], priorityBit: Bit, index: number = 0): string => {
  if (codes.length === 1) return codes[0] ?? ''

  const bit = bitPredicate === 'mcb'
    ? mostCommonBit(codes.map(c => parseInt(c[index] ?? '0') as Bit))
    : mostCommonBit(codes.map(c => parseInt(c[index] ?? '0') as Bit)) === 0 ? 1 : 0

  if (codes.length % 2 === 0) {
    const bitCount = codes
      .map(c => c[index])
      .reduce((accumulator, bit) => parseInt(bit ?? '0') === 0
        ? {
            ...accumulator,
            zero: accumulator.zero + 1
          }
        : {
            ...accumulator,
            one: accumulator.one + 1
          }
      , { zero: 0, one: 0 })

    if (bitCount.one === bitCount.zero) {
      return parseMatchingCode(bitPredicate, codes.filter(c => c[index] === priorityBit.toString()), priorityBit, index + 1)
    }
  }

  return parseMatchingCode(bitPredicate, codes.filter(c => c[index] === bit.toString()), priorityBit, index + 1)
}

export const parseDiagnostic = (input: string[]): Diagnostic => {
  const rateLength = Math.min(
    ...input
      .filter(x => x !== '')
      .map(x => x.length)
  )
  const gammaRate = Array(rateLength)
    .fill(0)
    .map((_, i) => mostCommonBit(input.map(x => parseInt(x[i] ?? '0') as Bit)))
    .join('')

  const epsilonRate = gammaRate.split('').map(x => x === '0' ? '1' : '0').join('')

  const oxygenRating = parseMatchingCode('mcb', input, 1)
  const co2ScrubberRating = parseMatchingCode('lcb', input, 0)

  const powerConsumption = parseInt(gammaRate, 2) * parseInt(epsilonRate, 2)
  const lifeSupportRating = parseInt(oxygenRating, 2) * parseInt(co2ScrubberRating, 2)

  return {
    raw: input,
    gammaRate,
    epsilonRate,
    oxygenRating,
    co2ScrubberRating,
    powerConsumption,
    lifeSupportRating
  }
}
