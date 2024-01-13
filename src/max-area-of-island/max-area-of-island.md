

----

Tags: #matrix #leetcode #medium #graph

----

## Drawing:
[[max-area-of-island.excalidraw]]

----


## solution explanation:
run dfs on each cell calculating area of island and add cel to visit

## last submission:
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
var maxAreaOfIsland = function (grid) {
    const height = grid.length
    const width = grid[0].length
    const dfs = (x, y) => {
        if (x < 0 || x >= height || y < 0 || y >= width || grid[x][y] === 0) return 0
        grid[x][y] = 0
        let amount = 1
        amount += dfs(x + 1, y)
        amount += dfs(x - 1, y)
        amount += dfs(x, y + 1)
        amount += dfs(x, y - 1)
        return amount
    }
    let max = 0
    for (let x = 0; x < height; x++) {
        for (let y = 0; y < width; y++) {
            if (grid[x][y] === 0) continue
            const res = dfs(x, y)
            if (res > max) {
                max = res
            }

        }
    }
    return max
};
```



