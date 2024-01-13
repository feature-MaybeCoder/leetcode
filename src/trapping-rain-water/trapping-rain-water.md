

----

Tags: #array #dp #hard #leetcode #stack #sliding-window #breath-first-search

----

## Drawing:
[[trapping-rain-water.excalidraw]]

----


## solution1 explanation:

calculate max from left to right and from right to left, find min for each index, min is represent max amount of water that we can store at that index, actual amount is diff between min and index height

## last submission:
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function (height) {

    let maxsLeft = []
    let maxsRight = []
    let minRightLeft = []
    let curMax = 0
    for (let i = 0; i < height.length; i++) {
        const curHeight = height[i]
        maxsLeft.push(curMax)
        if (curHeight > curMax) {
            curMax = curHeight
        }
    }
    curMax = 0
    for (let i = height.length - 1; i >= 0; i--) {
        const curHeight = height[i]
        maxsRight.push(curMax)
        if (curHeight > curMax) {
            curMax = curHeight
        }
    }
    maxsRight = maxsRight.reverse()
    for (let i = 0; i < height.length; i++) {
        minRightLeft.push(Math.min(maxsRight[i], maxsLeft[i]))
    }

    let sum =0
    for (let i = 0; i < height.length; i++) {
        const curHeight = height[i]
        const curMin = minRightLeft[i]
        if(curHeight < curMin){
            sum += curMin - curHeight
        }
    }


    return sum
};
```

## solution 2 explanation:
two pointers solution, start at start end en of the array, shift the pointer that have the min maximum height, actual amount of water that we can store at each pointer is the diff between height and min max of each pointer

## two pointers solution:
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function (height) {

    let sum = 0
    let leftPointer = 0
    let rightPointer = height.length - 1

    let leftPointerMax = 0
    let rightPointerMax = 0

    while (leftPointer <= rightPointer) {
        const leftHeight = height[leftPointer]
        const rightHeight = height[rightPointer]
        let minOfMax
        if (leftHeight > leftPointerMax) {
            leftPointerMax = leftHeight
        }
        if (rightHeight > rightPointerMax) {
            rightPointerMax = rightHeight
        }
        if (leftPointerMax >= rightPointerMax) {
            minOfMax = rightPointerMax
            rightPointer--
        } else {
            minOfMax = leftPointerMax
            leftPointer++
        }

        if (leftHeight < minOfMax) {
            sum += minOfMax - leftHeight
        }
        if (rightHeight < minOfMax) {
            sum += minOfMax - rightHeight
        }

        

    }

    return sum
};
```