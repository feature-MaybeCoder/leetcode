

----

Tags: #graph #matrix #medium #leetcode

----

## Drawing:
[[number-of-closed-islands.excalidraw]]

----


## solution explanation:
run dfs on edges of matrix, then iterating over all rest matrix if we not visit this cell yet increment amount, than continue

## last submission:
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
var closedIsland = function (grid) {
    const height = grid.length
    const width = grid[0].length
    const dfs = (x, y) => {
        if (x >= height || x < 0 || y < 0 || y >= width || grid[x][y] === 1) return
        grid[x][y] = 1
        dfs(x + 1, y)
        dfs(x - 1, y)
        dfs(x, y - 1)
        dfs(x, y + 1)
    }
    for (let x = 0; x < height; x++) {
        dfs(x, 0)
        dfs(x, width - 1)
    }
    for (let y = 0; y < width; y++) {
        dfs(0, y)
        dfs(height - 1, y)
    }
    let amount = 0
    for (let x = 0; x < height; x++) {
        for (let y = 0; y < width; y++) {
            const val = grid[x][y]
            if (val === 0) amount++
            dfs(x, y)

        }
    }
    return amount
};
```


