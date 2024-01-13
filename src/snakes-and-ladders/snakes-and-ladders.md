

----

Tags: #matrix #graph #depth-first-search #breath-first-search #medium #leetcode

----

## Drawing:
[[snakes-and-ladders.excalidraw]]

----


## solution explanation:
basic bfs with visited set if we already visit node, that means that current path is equal or larger than previous
#algo-note use bfs for shorter paths kind problems

## last submission:
```javascript
/**
 * @param {number[][]} board
 * @return {number}
 */
var snakesAndLadders = function (board) {
  const len = board.length
  const maxAmount = len * len
  let visitedSet = new Set()
  const n = board.length;

  const getRowColFromNode = (node) => {
    let row = Math.ceil(node / len)
    let rowIndex = row - 1
    let isoOdd = (row) % 2
    let col = (len * row) - node
    if (isoOdd) {
      return [len - 1 - rowIndex, (len - col) - 1]

    }
    return [len - 1 - rowIndex, col]


  }

  const stack = [[1, 0]]

  while (stack.length) {
    const [node, depth] = stack.shift()

    if (node === maxAmount) {
      return depth

    }
    for (let i = 1; i <= 6; i++) {
      let newNode = node + i
      let newDepth = depth + 1
      if (newNode > maxAmount) break
      const [row, col] = getRowColFromNode(newNode)
      const boardValue = board[row][col]

      if (boardValue !== -1) {
        newNode = boardValue
      }
      if (!visitedSet.has(newNode)) {
        visitedSet.add(newNode);

        stack.push([newNode, newDepth])
      }

    }
  }


  return -1
};
```



