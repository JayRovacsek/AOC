export type Segment = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g'

export type Display = {
    value: number,
    segments: Segment[]
}

export const parseDisplayV1 = (input: Segment[]): Display | null => {
  if (input.length === 2) {
    return {
      value: 1,
      segments: input
    }
  }

  if (input.length === 3) {
    return {
      value: 7,
      segments: input
    }
  }

  if (input.length === 4) {
    return {
      value: 4,
      segments: input
    }
  }

  if (input.length === 7) {
    return {
      value: 8,
      segments: input
    }
  }

  return null
}

export const parseDisplayV2 = (data: {input: string[], output: string[]}): Display => {
  const one = data.input
    .find(x => x.length === 2) ?? ''

  const seven = data.input
    .find(x => x.length === 3) ?? ''

  const four = data.input
    .find(x => x.length === 4) ?? ''

  const eight = data.input
    .find(x => x.length === 7) ?? ''

  const nine = data.input
    .filter(x => x.length === 6)
    .find(x => seven.split('')
      .every(v => x.includes(v)) && four.split('')
      .every(v => x.includes(v))) ?? ''

  const zero = data.input
    .filter(x => x.length === 6)
    .find(x => x !== nine && seven.split('')
      .every(v => x.includes(v))) ?? ''

  const six = data.input
    .filter(x => x.length === 6)
    .find(x => x !== nine && x !== zero) ?? ''

  const three = data.input
    .filter(x => x.length === 5)
    .find(x => seven.split('')
      .every(v => x.includes(v))) ?? ''

  const topLhs = six.split('')
    .filter(x =>
      three.split('').includes(x) === false &&
    eight.split('').includes(x) &&
    four.split('').includes(x)
    )[0] ?? ''

  const five = data.input
    .filter(x => x.length === 5)
    .find(x => x !== three && x.includes(topLhs)) ?? ''

  const two = data.input
    .filter(x => x.length === 5)
    .find(x => x !== three && x !== five) ?? ''

  const numbers = [zero, one, two, three, four, five, six, seven, eight, nine]
    .map((x, i) => ({ value: i, raw: x }))

  return {
    value: parseInt(data.output
      .map(x => numbers
        .find(n => n.raw === x)?.value ?? '')
      .join('')),
    segments: data.output as Segment[]
  }
}
