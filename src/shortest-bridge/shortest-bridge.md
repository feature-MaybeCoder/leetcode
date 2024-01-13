

----

Tags: #graph #leetcode #medium #breath-first-search

----

## Drawing:
[[shortest-bridge.excalidraw]]

----


## solution explanation:
run dfs on first visited land, push to stack each cell of founded island, run bfs from each cel of founded island, on first visit another island return depth #tip modify matrix with separated values for visited management 

## last submission:
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
var shortestBridge = function (grid) {
    const width = grid[0].length - 1
    const height = grid.length - 1
    const stack = []
    const dirs = [0, 1, 0, -1, 0]
    const dfs = (x, y) => {
        const val = grid[x][y]
        if (val === 2 || val === 0) return
        grid[x][y] = 2
        stack.push([x, y, 0])
        for (let i = 0; i < 4; i++) {
            const dx = x + dirs[i]
            const dy = y + dirs[i + 1]
            if (dx > height || dx < 0 || dy > width || dy < 0) continue
            dfs(dx, dy)

        }
    }
    outer: for (let x = 0; x <= height; x++) {
        for (let y = 0; y <= width; y++) {
            if (grid[x][y] === 0) continue
            dfs(x, y)
            break outer
        }
    }

    let min = Infinity

    while (stack.length) {
        let [x, y, depth] = stack.shift()
        const val = grid[x][y]
        if (val === 3) continue
        if (val === 1) {
            return depth - 1
        }
        grid[x][y] = 3
        depth += 1
        for (let i = 0; i < 4; i++) {
            const dx = x + dirs[i]
            const dy = y + dirs[i + 1]
            if (dx > height || dx < 0 || dy > width || dy < 0) continue

            stack.push([dx, dy, depth])
        }
    }
    return min
};
```



