

----

Tags: #stack #medium #leetcode

----

## Drawing:
[[longest-consecutive-sequence.excalidraw]]

----


## solution explanation:
implement union find solution: only on new item check is lhs (-1) rhs(+1) item in map, if it is, union them, set max length to maximum of curent and new

hashset solution: add all nums to set, then iterate over all starts sequence, the start of sequence is num that does not have n-1 value, iterate to all squencial numbers n+1 and set length to maximum

## last submission:
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function (nums) {
    let set = new Set(nums)
    let maxStreak = 0

    for (let num of set) {
        if (set.has(num - 1)) continue;
        let curStreak = 1
        let nextNum = num + 1
        while (set.has(nextNum)) {
            curStreak++
            nextNum++
        }

        maxStreak = Math.max(curStreak, maxStreak)
    }
    return maxStreak
};
```
## dfs solution:
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function (nums) {
    if (!nums.length) return 0
    let parents = {}
    let weights = {}
    let maxLength = 1
    let find = (node) => {
        while (node != parents[node]) {
            node = parents[node] = parents[parents[node]]
        }
        return node
    }
    let union = (node1, node2) => {
        let p1 = find(node1)
        let p2 = find(node2)
        parents[p2] = p1
        maxLength = Math.max(weights[p2] = weights[p1] += weights[p2], maxLength)

    }
    for (let num of nums) {

        if (!(num in weights)) {
            weights[num] = 1
            parents[num] = num
        } else {
            continue
        }
        let lhs = num - 1
        let rhs = num + 1
        if (lhs in weights) {
            union(num, lhs)
        }
        if (rhs in weights) {
            union(num, rhs)
        }
    }
    return maxLength
};
```


