

----

Tags: #array #array #dp #hard #leetcode #repeat

----

## Drawing:
[[median-of-two-sorted-arrays.excalidraw]]

----


## solution explanation:
merge only left half of two arrays
## optimizations:
merge only indexes would simplify memory complexity to constant 
## last submission:
```javascript
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
var findMedianSortedArrays = function (nums1, nums2) {
    const sum = nums1.length + nums2.length
    let stopMergeAt = Math.round(sum / 2)

    const merged = []
    let i = 0
    let p = 0
    const isNotOdd = sum % 2
    if (!isNotOdd) {
        stopMergeAt += 1
    }
    while (i + p < stopMergeAt) {
        const num1 = nums1[i]
        const num2 = nums2[p]
        if (num1 === undefined || (num2 !== undefined && num1 > num2)) {
            merged.push(num2)
            p++
            continue
        }
        merged.push(num1)
        i++
    }

    if (isNotOdd) {
        return merged[stopMergeAt - 1]
    }
    return (merged[stopMergeAt - 1] + merged[stopMergeAt - 2]) / 2
};
```


