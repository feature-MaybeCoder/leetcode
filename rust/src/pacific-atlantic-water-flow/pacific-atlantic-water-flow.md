

----

Tags: #matrix #leetcode #medium #graph #interesting #repeat 

----

## Drawing:
[[pacific-atlantic-water-flow.excalidraw]]

----


## solution explanation:
two solutions:
ineffective: start dfs at each point iterating while we reach some border temp add cell to visited for dealing with recursions, add to visited is we can reach two borders timeComplexity: 0^2
effective" start dfs at borders, with to visited sets pacific and atlantic move to cell if its larger or equal, if we reach cell add it to corresponding visited set, if cell in both sets, than from this cell we can reach atlantic and pacific oceans

## last submission:
```javascript
/**
 * @param {number[][]} heights
 * @return {number[][]}
 */
var pacificAtlantic = function (heights) {
    const height = heights.length
    const _height = heights.length - 1
    const width = heights[0].length
    const _width = heights[0].length - 1
    const dirs = [0, 1, 0, -1, 0]
    let visitedp = Array.from({ length: height * width }, () => false)
    let visitedc = Array.from({ length: height * width }, () => false)
    const getIndex = (x, y) => (x * width) + y
    const dfs = (x, y, v) => {
        const index = getIndex(x, y)
        let isVisited = v[index]
        if (isVisited) return
        v[index] = true

        let value = heights[x][y]

        for (let i = 0; i < 4; i++) {
            const dx = x + dirs[i]
            const dy = y + dirs[i + 1]
            if (dx < 0 || dx >= height || dy < 0 || dy >= width) continue
            const newVal = heights[dx][dy]
            if (newVal < value) continue
            dfs(dx, dy, v)
        }

    }
    for (let x = 0; x < height; x++) {
        dfs(x, 0, visitedp)
        dfs(x, _width, visitedc)
    }
    for (let y = 0; y < width; y++) {
        dfs(0, y, visitedp)
        dfs(_height, y, visitedc)
    }

    let reses = []
    for (let x = 0; x < height; x++) {
        for (let y = 0; y < width; y++) {
            const index = getIndex(x, y)

            if (visitedp[index] && visitedc[index]) {
                reses.push([x, y])
            }
        }
    }
    return reses
};
```



