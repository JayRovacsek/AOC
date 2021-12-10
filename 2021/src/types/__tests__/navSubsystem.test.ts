/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
import { parseNavigationSubsystem } from '../navSubsystem'

const testInput = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`

describe('NavSubsystem', () => {
  it('should subsystem correctly', async () => {
    const subsystem = parseNavigationSubsystem(testInput)
    expect(subsystem.corrupt.length).toEqual(5)
  })
})
