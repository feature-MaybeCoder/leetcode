

----

Tags: #matrix #graph #leetcode #medium #repeat #beats-all

----

## Drawing:
[[01-matrix.excalidraw]]

----


## solution explanation:
#tip start bfs at all possible position for evaluating range distance. #tip iterating over stack and replace array for slice to slice technique, that will remove shift time complexity for each iteration.
#tip start two loops from top to bottom from bottom to top, merge left top, and right bottom for matrix evaluation 

## last submission:
```javascript
/**
 * @param {number[][]} mat
 * @return {number[][]}
 */
var updateMatrix = function (mat) {
    const height = mat.length
    const width = mat[0].length
    const firstDirs = [0, -1, 0]
    for (let x = 0; x < height; x++) {
        for (let y = 0; y < width; y++) {

            const val = mat[x][y]
            if (val === 0) continue
            let newValue = Infinity
            for (let i = 0; i < 2; i++) {
                const dx = x + firstDirs[i]
                const dy = y + firstDirs[i + 1]
                if (dx < 0 || dy < 0) continue
                const v = mat[dx][dy] + 1
                if (v < newValue) {
                    newValue = v
                }
            }
            mat[x][y] = newValue
        }
    }
    
    const secondDirs = [0, 1, 0]
    for (let x = height - 1; x >= 0; x--) {
        for (let y = width - 1; y >= 0; y--) {
            
            let val = mat[x][y]
            if (val === 0) continue
            let newValue = mat[x][y]
            for (let i = 0; i < 2; i++) {
                const dx = x + secondDirs[i]
                const dy = y + secondDirs[i + 1]
                if (dx >= height || dy >= width) continue
                const v = mat[dx][dy] + 1
                if (v < newValue) {
                    newValue = v
                }
            }
            mat[x][y] = newValue
        }
    }
    return mat
};
```



