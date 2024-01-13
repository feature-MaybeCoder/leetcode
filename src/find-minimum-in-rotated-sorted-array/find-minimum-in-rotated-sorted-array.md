

----

Tags: #array #leetcode #medium #binary-search  #find-peak

----

## Drawing:
[[find-minimum-in-rotated-sorted-array.excalidraw]]

----


## solution explanation:
two possible solutions: find min pick solution check is piwot el is le than prev and next to it, if it is than we found min element, else, if end element is ge than piwot move to left else, move to right

second algo: found needed subarray and return last element, if array is rotated N times, that means we have two subarrays one is le than other, we need to found smaller array and return last element

we in sorted not rotated portion if left el is smaller than right el, return min of res and cur left element

find piwot, update to min of piwot and cur rest if piwot is GE than start move to right, else move to left

## last submission:
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var findMin = function (nums) {
    let start = 0
    let end = nums.length - 1

    let res = nums[0]
    while (end >= start) {
        if (nums[start] < nums[end]) {
            res = Math.min(nums[start], res)
            break
        }
        let piwot = end + start >> 1
        res = Math.min(res, nums[piwot])
        if (nums[piwot] >= nums[start]) {
            start = piwot + 1
            continue
        }
        end = piwot - 1


    }

    return res
};
```

```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var findMin = function (nums) {
    let start = 0
    let end = nums.length - 1

    let piwot = end + start >> 1
    while (end >= start) {
        
        let next = nums[piwot+1]
        let prev = nums[piwot-1]
        if(next === undefined)next = Infinity
        if(prev === undefined)prev = Infinity
        if(nums[piwot] < next && nums[piwot] < prev)return nums[piwot]
        if (nums[end] > nums[piwot]) {
            end = piwot-1
            piwot = end + start >> 1
            continue
        }
        start = piwot+1
        piwot = end + start >> 1
    }
    return nums[piwot]
};
```

