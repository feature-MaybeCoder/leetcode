

----

Tags: #binary-search #leetcode #array #medium

----

## Drawing:
[[find-peak-element.excalidraw]]

----


## solution explanation:
basic binary search solution, if right el is lg than el move to right, else to left or return cur piwot

## last submission:
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var findPeakElement = function (nums) {

    let start = 0
    let end = nums.length - 1
    while (end >= start) {
        let piwot = end + start >> 1
        const el = nums[piwot]
        const right = nums[piwot + 1] || -Infinity
        const left = nums[piwot - 1] || -Infinity
        if (el > right && el > left) return piwot
        if (right > el) {
            start = piwot + 1
            continue
        }
        end = piwot - 1
    }
};
```



