import { parseBingoCalls, Bingo } from '../classes/bingo'
import { Day } from '../interfaces/day'

export const four: Day = {
  day: 4,
  partOne: async (input: string): Promise<string> => {
    const calls = parseBingoCalls(input.split('\n')[0] ?? '0')
    const boards = input
      .split('\n\n')
      .filter((_, i) => i !== 0)
      .map(x => new Bingo(x.split('\n')))

    const lastCall = calls.find((call, i) => {
      boards.forEach(board => board.applyCall(call))
      if (i > 4 && boards.some(board => board.isComplete())) return true
      return false
    })
    const completeBoard = boards.find(board => board.isComplete())
    const tileSum = completeBoard?.sumOfTiles() ?? 0
    return `${tileSum * (lastCall ?? 0)}`
  },

  partTwo: async (input: string): Promise<string> => {
    const calls = parseBingoCalls(input.split('\n')[0] ?? '0')
    const boards = input
      .split('\n\n')
      .filter((_, i) => i !== 0)
      .map(x => new Bingo(x.split('\n')))

    const lastCall = calls.find((call, i) => {
      boards.forEach(board => board.applyCall(call))
      boards
        .filter(board => board.isComplete() && board.completionRound === 0)
        .forEach(board => board.setCompletionRound(i))
      if (i > 4 && boards.every(board => board.isComplete())) return true
      return false
    })
    const lastCompleteBoard = boards.sort((a, b) => b.completionRound - a.completionRound)[0]
    const tileSum = lastCompleteBoard?.sumOfTiles() ?? 0
    return `${tileSum * (lastCall ?? 0)}`
  }
}
