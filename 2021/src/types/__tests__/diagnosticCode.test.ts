/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { parseDiagnostic } from '../diagnosticCode'

const testDiagnostic = `00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010`

describe('Utils', () => {
  it('should parse diagnostic codes correctly', async () => {
    const codes = testDiagnostic.split('\n')
    const diagnostic = parseDiagnostic(codes)
    expect(diagnostic.gammaRate).toEqual('10110')
    expect(diagnostic.epsilonRate).toEqual('01001')
    expect(diagnostic.oxygenRating).toEqual('10111')
    expect(diagnostic.co2ScrubberRating).toEqual('01010')
    expect(diagnostic.powerConsumption).toEqual(198)
    expect(diagnostic.lifeSupportRating).toEqual(230)
  })
})
