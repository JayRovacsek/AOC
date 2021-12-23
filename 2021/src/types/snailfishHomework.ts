import { SnailfishNumber } from './snailfishNumber'

export type Homework = {
    [key: number]: SnailfishNumber[]
}

export const parseNumericValues = (input: string): number[] => {
  const chars = input
    .split('')
    .reduce((accumulator: {values: string[], valid: boolean}, x) => {
      if (/[0-9,]/.test(x) && accumulator.valid) return { ...accumulator, values: [...accumulator.values, x] }
      return { ...accumulator, valid: false }
    }, { valid: true, values: [] }).values

  return [...chars].filter(x => x !== ',').map(x => parseInt(x))
}

export const parseHomework = (input: string, homework?: Homework, index?: number): Homework => {
  if (homework === undefined) {
    const initialSplit = input.substring(1, input.length - 1)
    const reversed = initialSplit.startsWith('[')
    if (reversed) {
      const values = parseNumericValues(initialSplit
        .split('')
        .reverse()
        .join('')
      ).reverse()
      return parseHomework(initialSplit, { 0: values }, 1)
    }
    const values = parseNumericValues(initialSplit)
    return parseHomework(initialSplit, { 0: values }, 1)
  }

  if (input.includes('[') === false || input.includes(']') === false) {
    return Object.fromEntries(
      Object.entries(homework)
        .filter(([_, value]) => value.length > 0)
    )
  }

  const maxIndex = index !== undefined ? index : top(homework).index + 1
  const reversedInput = input.split('').reverse().join('')
  const nextLevelStart = input.search(/\[/)
  const nextLevelEnd = input.length - (1 + reversedInput.search(/\]/))

  const trimmed = input.substring(nextLevelStart + 1, nextLevelEnd)
  const reversed = trimmed.startsWith('[')

  if (reversed) {
    return parseHomework(trimmed, {
      ...homework,
      [maxIndex]: parseNumericValues(trimmed
        .split('')
        .reverse()
        .join('')
      ).reverse()
    }, maxIndex + 1)
  }

  return parseHomework(trimmed, { ...homework, [maxIndex]: parseNumericValues(trimmed) }, maxIndex + 1)
}

const parseValidChars = (input: string[]): string => {
  return input
    .reduce(
      (accumulator: { valid: boolean, stack: string[], chars: string[] }, x) => {
        if (accumulator.valid) {
          if (x === '[' && accumulator.stack[accumulator.stack.length - 1] === '[') {
            return {
              ...accumulator, valid: false
            }
          }

          if (x === ']' && accumulator.stack[accumulator.stack.length - 1] === '[') {
            return {
              ...accumulator, stack: [accumulator.stack.filter((_, i) => i <= accumulator.stack.length - 2)]
            }
          }

          if (x === ']') {
            return {
              ...accumulator, valid: false
            }
          }

          return {
            ...accumulator,
            chars: [...accumulator.chars, x]
          }
        }

        return accumulator
      }
      , { valid: true, stack: [], chars: [] }
    )
}

const parseHomeworkLevel = (input: string): string => {
  const chars = input.split('')
  const forward = chars.reduce((accumulator, x) => {
    if (accumulator.valid) {
      if (x === '[' && accumulator.stack[accumulator.stack.length - 1] === '[') {
        return {
          ...accumulator, valid: false
        }
      }

      if (x === ']' && accumulator.stack[accumulator.stack.length - 1] === '[') {
        return {
          ...accumulator, stack: [accumulator.stack.filter((_, i) => i <= accumulator.stack.length - 2)]
        }
      }

      if (x === ']') {
        return {
          ...accumulator, valid: false
        }
      }

      return {
        ...accumulator,
        chars: [...accumulator.chars, x]
      }
    }

    return accumulator
  }, { valid: true, stack: [], chars: [] })
}

export const explodeApplies = (input: Homework): boolean => {
  return Object.keys(input).some((key) => parseInt(key) >= 4)
}

export const top = (input: Homework): { index: number, value: SnailfishNumber } => {
  const topLevel = Math.max(...Object.keys(input).map(x => parseInt(x)))

  /**
   * Safe to ignore the below as the only instance this should fail is
   * when an empty object is passed or keys don't adhere to our type
   * which would be caught by the type checker
   */
  // @ts-ignore
  return nth(input, topLevel)
}

export const nth = (input: Homework, index: number): { index: number, value: SnailfishNumber } | null => {
  const indices = Object.keys(input).length

  if (index > indices) return null

  if (index >= Math.floor(indices / 2)) {
    return {
      index,
      value: Object
        .entries(input)
        .reverse()
        .find(([key, _]) => parseInt(key) === index)?.[1] ?? 0
    }
  }
  return {
    index,
    value: Object
      .entries(input)
      .find(([key, _]) => parseInt(key) === index)?.[1] ?? 0
  }
}
