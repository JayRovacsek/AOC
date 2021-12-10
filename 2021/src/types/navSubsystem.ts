const open = { '(': ')', '[': ']', '{': '}', '<': '>' }
const openScoreMap = { ')': 1, ']': 2, '}': 3, '>': 4 }
const close = { ')': '(', ']': '[', '}': '{', '>': '<' }
const closeScoreMap = { ')': 3, ']': 57, '}': 1197, '>': 25137 }

export type NavigationSubsystem = {
    raw: string,
    incomplete: string[]
    corrupt: string[],
    completionSequences: string[][]
    illegalChars: string[],
    syntaxErrorScore: number
    completionScore: number
}

const validateLine = (input: string): boolean => {
  const chars = input.split('')
  return chars.reduce((accumulator: {valid: boolean, state: string[]}, c) => {
    if (accumulator.valid === false) return accumulator
    if (c in open) {
      return {
        ...accumulator,
        state: [...accumulator.state, c]
      }
    }
    if (c in close && Object.entries(open).find(([k, v]) => v === c && k === accumulator.state[accumulator.state.length - 1]) !== undefined) {
      return {
        ...accumulator,
        state: accumulator.state.filter((_, i) => i !== accumulator.state.length - 1)
      }
    }
    return {
      ...accumulator,
      valid: false
    }
  }, { valid: true, state: [] }).valid
}

const findInvalidChar = (input: string): string => {
  const chars = input.split('')
  return chars.reduce((accumulator: {state: string[], invalidChar: string}, c) => {
    if (accumulator.invalidChar !== '') return accumulator
    if (c in open) {
      return {
        ...accumulator,
        state: [...accumulator.state, c]
      }
    }
    if (c in close && Object.entries(open).find(([k, v]) => v === c && k === accumulator.state[accumulator.state.length - 1]) !== undefined) {
      return {
        ...accumulator,
        state: accumulator.state.filter((_, i) => i !== accumulator.state.length - 1)
      }
    }
    return {
      ...accumulator,
      invalidChar: c
    }
  }, { state: [], invalidChar: '' }).invalidChar
}

const completeSequence = (input: string): string[] => {
  const chars = input.split('')
  const { state } = chars.reduce((accumulator: {state: string[], invalidChar: string}, c) => {
    if (accumulator.invalidChar !== '') return accumulator
    if (c in open) {
      return {
        ...accumulator,
        state: [...accumulator.state, c]
      }
    }
    if (c in close && Object.entries(open).find(([k, v]) => v === c && k === accumulator.state[accumulator.state.length - 1]) !== undefined) {
      return {
        ...accumulator,
        state: accumulator.state.filter((_, i) => i !== accumulator.state.length - 1)
      }
    }
    return {
      ...accumulator,
      invalidChar: c
    }
  }, { state: [], invalidChar: '' })

  // @ts-ignore
  return state.reverse().map(x => open[x])
}

export const parseNavigationSubsystem = (input: string): NavigationSubsystem => {
  const lines = input.split('\n').filter(x => x.length > 0)
  const incomplete = lines
    .filter(line => validateLine(line))

  const corrupt = lines
    .filter(line => validateLine(line) === false)

  const illegalChars = corrupt.map(line => findInvalidChar(line))
  const completionSequences = incomplete.map(line => completeSequence(line))

  const syntaxErrorScore = illegalChars
    // @ts-ignore
    .reduce((accumulator, c) => accumulator + (closeScoreMap[c] ?? 0), 0)

  const completionScores = completionSequences
    .map(set => set
      .reduce((accumulator, c) => {
        // @ts-ignore
        return (accumulator * 5) + (openScoreMap[c] ?? 0)
      }, 0)
    ).sort((a, b) => a - b)

  const completionScore = completionScores
    .find((_, i) => i === Math.floor(completionScores.length / 2)) ?? 0

  return {
    raw: input,
    incomplete,
    corrupt,
    illegalChars,
    completionSequences,
    syntaxErrorScore,
    completionScore
  }
}
