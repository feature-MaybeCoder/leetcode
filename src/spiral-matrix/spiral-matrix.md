

----

Tags: #matrix #medium #leetcode

----

## Drawing:
[[spiral-matrix.excalidraw]]

----


## solution explanation:
#tip: working with matrixes init fore vars on left,right,top,bottom increment or decrement with needed rules

## last submission:
```javascript
/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
var spiralOrder = function (matrix) {
    const items = []
    const width = matrix[0].length - 1
    const height = matrix.length - 1

    let left = 0
    let right = width
    let top = 0
    let bottom = height
    while (left <= right && top <= bottom) {

        for (let i = left; i <= right; i++) {
            const item = matrix[top][i]

            items.push(item)
        }
        for (let i = top + 1; i <= bottom; i++) {
            const item = matrix[i][right]

            items.push(item)
        }
        for (let i = right - 1; i > left && bottom !== top; i--) {
            const item = matrix[bottom][i]

            items.push(item)
        }
        for (let i = bottom; i > top && left !== right; i--) {
            const item = matrix[i][left]

            items.push(item)
        }

        left++
        right--
        top++
        bottom--

    }
    return items
};
```



