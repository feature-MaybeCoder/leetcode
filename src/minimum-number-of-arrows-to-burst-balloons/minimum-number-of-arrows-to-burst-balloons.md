

----

Tags: #intervals #medium #leetcode

----

## Drawing:
[[minimum-number-of-arrows-to-burst-balloons.excalidraw]]

----


## solution explanation:
sort intervals by second position, impl sliding window aproach 

## last submission:
```javascript
/**
 * @param {number[][]} points
 * @return {number}
 */
var findMinArrowShots = function (points) {
    points.sort((a, b) => a[1] - b[1])
    let amount = points.length
    let left = 0
    let right = 0

    while (left < points.length) {
        let leftInterval = points[left]
        right++
        while (right < points.length) {
            let rightInterval = points[right]
            
            if (leftInterval[1] >= rightInterval[0]) {
                right++
                continue
            }
            break
        }
        let dif = (right - left) - 1
        amount -= dif
        left = right

    }

    return amount
};
```



