export type CrabSubmarine = {
    position: number
}

export const parseCrabSumbraine = (input: string): CrabSubmarine => {
  return {
    position: parseInt(input)
  }
}
