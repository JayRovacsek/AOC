const upperRegex = /[A-Z]+/

type TunnelSize = 'small' | 'large'

type Tunnel = {
    size: TunnelSize,
    name: string
    isStart: boolean,
    isEnd: boolean,
}

type Node = {
    t1: Tunnel,
    t2: Tunnel
}

const parseTunnel = (input: string): Tunnel => {
  const isStart = input === 'start'
  const isEnd = input === 'end'
  const size = upperRegex.test(input) ? 'large' : 'small'
  return {
    size: isStart || isEnd ? 'large' : size,
    name: input,
    isStart,
    isEnd
  }
}

const parseNode = (input: string): Node => {
  const parts = input.split('-')
  const t1 = parseTunnel(parts[0] ?? '')
  const t2 = parseTunnel(parts[1] ?? '')

  return {
    t1,
    t2
  }
}

const parsePaths = (nodes: Node[], start?: Node, path?: Node[]): Node[][] => {
  /**
   * If a start isn't defined, find all nodes that have t1 element of start and
   * branch for each of the options flattening at the end
   */
  if (start === undefined) {
    const startNodes = nodes.filter(x => x.t1.isStart)
    return startNodes
      .map(x =>
        parsePaths(nodes.filter(y => y.t1.isStart === false), x)
      )
      .flat()
  }

  /**
   * If start is defined, but a path is not, we know this must be a first run,
   * take all nodes that match as a joining element of the start element and
   * branch
   */
  if (path === undefined) {
    const initialSteps = nodes.filter(x => x.t1.name === start.t2.name)
    return initialSteps
      .map(x =>
        parsePaths(nodes, start, [start, x])
      )
      .flat()
  }

  const traversedSmallCaves = Array.from(
    new Set(
      path
        .filter(x => x.t1.size === 'small')
        .map(x => x.t1.name)
    )
  )

  const lastStop = path[path.length - 1]

  const pathIsFinished = lastStop?.t2.isEnd

  if (pathIsFinished) {
    return [path]
  }

  const branchingNodes = nodes.filter(x => x.t1.name === lastStop?.t2.name)

  const validUnfinishedNodes = branchingNodes.filter(x => {
    if (x.t2.size === 'small') {
      return traversedSmallCaves.every(cave => cave !== x.t2.name)
    }
    return true
  })

  const finishedPaths = branchingNodes.filter(x => x.t2.isEnd).map(x => [...path, x])

  return [
    ...finishedPaths, ...validUnfinishedNodes
      .map(x => parsePaths(nodes, start, [...path, x]))
      .flat()
  ]
    .filter(x => x.length > 0)
}

export class Cave {
    raw: string;
    nodes: Node[];
    paths: Node[][];
    pathstrings: string[];

    constructor (input: string) {
      this.raw = input
      const nodes: Node[] = input.split('\n').map(x => parseNode(x))
      const reversedNodes: Node[] = nodes
        .filter(x => x.t1.isStart === false)
        .map(x => ({
          t1: x.t2,
          t2: x.t1
        }))

      this.nodes = [...nodes, ...reversedNodes]

      const paths = parsePaths(this.nodes)

      this.paths = paths

      const pathStrings = paths
        .map(x => `${x.map(y => y.t1.name).join(',')},end`
        )

      const dedupedPaths = Array.from(new Set(pathStrings))
      this.pathstrings = dedupedPaths
      this.pathstrings = dedupedPaths
    }
}
