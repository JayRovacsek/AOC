export type CrabSubmarine = {
    position: number
}

export const parseCrabSubmarine = (input: string): CrabSubmarine => {
  return {
    position: parseInt(input)
  }
}
