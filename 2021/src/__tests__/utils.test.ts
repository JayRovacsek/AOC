/* eslint-disable no-undef */
import { getInput } from '../utils'

describe('Utils', () => {
  it('should throw if we do not have a session cookie', async () => {
    process.env.SESSION_COOKIE = undefined
    expect(async () => await getInput(1)).rejects.toThrow('Missing session cookie from env variables.')
  })
})
