

----

Tags: #array #leetcode #medium #matrix #repeat 

----

## Drawing:
[[rotate-image.excalidraw]]

----


## solution explanation:
not optimal solution: calculate init stack for  top edge of every  even matrix with shfit, then calculate next index of element, push next position with temp value, rotate three times 

## last submission:
```javascript
const foundIndexesToSwap = (matrixIndex, indexInArr, matrixHeight) => {
    if (matrixIndex === 0) {
        return [indexInArr, matrixHeight - 1]
    }
    if (indexInArr === matrixHeight - 1) {
        return [matrixHeight - 1, matrixHeight - (matrixIndex + 1)]
    }
    if (matrixIndex === matrixHeight - 1) {
        return [indexInArr, 0]
    }
    return [0, matrixHeight - (matrixIndex + 1)]
}
/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
var rotate = function (matrix) {
    const temp = []
    for (let j = 0; j < matrix[0].length; j++) {
        temp.push([0, j, matrix[0][j], matrix.length])
    }
    let shift = matrix.length - 2
    let shiftIndex = 1
    while (shift > 1) {
        for (let j = 0; j < shift; j++) {
            temp.push([0, j, matrix[shiftIndex][j + shiftIndex], shift, shiftIndex])
        }
        shift -= 2
        shiftIndex++
    }

    while (temp.length) {
        const curTemp = temp.pop()
        const nextIndex = foundIndexesToSwap(curTemp[0], curTemp[1], curTemp[3])
        const indexShift = curTemp[4] || 0
        if (!(nextIndex[0] === 0)) {
            nextIndex.push(matrix[nextIndex[0] + indexShift][nextIndex[1] + indexShift], curTemp[3], curTemp[4])
            temp.push(nextIndex)
        }
        matrix[nextIndex[0] + indexShift][nextIndex[1] + indexShift] = curTemp[2]


    }
};
```

## optimal solution 
rotate matrix inplace by rotating in reverse order we need only one temp variable, calculate twio shift: first is current matrix, second is matrix corners that we need to rotate
