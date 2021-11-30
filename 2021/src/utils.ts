import axios from 'axios'

export const getInput = async (day: number): Promise<string> => {
  if (process.env.SESSION_COOKIE === 'undefined') throw new Error('Missing session cookie from env variables.')
  try {
    const request = await axios.get(`https://adventofcode.com/2021/day/${day}/input`, {
      headers: {
        Cookie: `session=${process.env.SESSION_COOKIE} `
      }
    })

    return request.data
  } catch (error) {
    if (typeof error === 'string') {
      console.error(error)
      throw new Error(error)
    }

    console.error(JSON.stringify(error, null, 4))
    throw new Error(JSON.stringify(error, null, 4))
  }
}
