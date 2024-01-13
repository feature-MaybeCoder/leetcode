

----

Tags: #depth-first-search #breath-first-search #graph #hard #leetcode

----

## Drawing:
[[longest-increasing-path-in-a-matrix.excalidraw]]

----


## solution explanation:
basic dfs path finding with visited cached

## last submission:
```javascript
/**
 * @param {number[][]} matrix
 * @return {number}
 */
var longestIncreasingPath = function (matrix) {
    const rowLen = matrix.length
    const colLen = matrix[0].length
    const visitedCache = new Map()
    let maxDepth = 0
    const getHashIndex = (rowIndex, colIndex) => rowIndex + " " + colIndex
    const dfs = (rowIndex, colIndex, fromOperation, depth = 1) => {
        const hashIndex = getHashIndex(rowIndex, colIndex)
        const cachedPath = visitedCache.get(hashIndex)
        if (cachedPath) {
            return cachedPath
        }
        const val = matrix[rowIndex][colIndex]
        let maxDepth = 0

        // left step
        if (fromOperation !== 'right' && colIndex > 0 && matrix[rowIndex][colIndex - 1] > val) {
            const dfsDepth = dfs(rowIndex, colIndex - 1, 'left')
            if (dfsDepth > maxDepth) {
                maxDepth = dfsDepth
            }
        }
        // right step
        if (fromOperation !== 'left' && colIndex < colLen - 1 && matrix[rowIndex][colIndex + 1] > val) {
            const dfsDepth = dfs(rowIndex, colIndex + 1, 'right')
            if (dfsDepth > maxDepth) {
                maxDepth = dfsDepth
            }
        }
        // top step
        if (fromOperation !== 'bottom' && rowIndex > 0 && matrix[rowIndex - 1][colIndex] > val) {
            const dfsDepth = dfs(rowIndex - 1, colIndex, 'top')
            if (dfsDepth > maxDepth) {
                maxDepth = dfsDepth
            }
        }
        // bottom step
        if (fromOperation !== 'top' && rowIndex < rowLen - 1 && matrix[rowIndex + 1][colIndex] > val) {
            const dfsDepth = dfs(rowIndex + 1, colIndex, 'bottom')
            if (dfsDepth > maxDepth) {
                maxDepth = dfsDepth
            }
        }
        depth += maxDepth
        visitedCache.set(hashIndex, depth)
        return depth

    }
    for (let rowIndex = 0; rowIndex < rowLen; rowIndex++) {
        for (let colIndex = 0; colIndex < colLen; colIndex++) {

            const depth = dfs(rowIndex, colIndex)

            if (depth > maxDepth) {
                maxDepth = depth
            }
        }
    }

    return maxDepth
};
```



