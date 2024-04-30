

----

Tags: #dp #medium #leetcode #array #intervals

----

## Drawing:
[[insert-interval.excalidraw]]

----


## solution explanation:
cover four main cases of insertion with oN complexity

## last submission:
```javascript
/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
var insert = function (intervals, newInterval) {
    let isPlaced = false
    if (!intervals.length) return [newInterval]
    for (let i = 0; i < intervals.length; i++) {
        const curInterval = intervals[i]
        if (curInterval[0] >= newInterval[0] && curInterval[1] <= newInterval[1]) {
            if (isPlaced) {
                intervals.splice(i, 1)
                i--
                continue
            }
            isPlaced = true
            curInterval[0] = newInterval[0]
            curInterval[1] = newInterval[1]
            continue
        }
        if (curInterval[0] <= newInterval[1] && newInterval[0] <= curInterval[0]) {
            const prevInterval = intervals[i - 1]
            if (prevInterval && prevInterval[1] > newInterval[0]) {
                intervals[i - 1][1] = curInterval[1]
                intervals.splice(i, 1)
                i--
                isPlaced = true
                continue
            }
            curInterval[0] = newInterval[0]
            continue
        }
        if (curInterval[1] <= newInterval[1] && curInterval[1] >= newInterval[0]) {

            curInterval[1] = newInterval[1]
            isPlaced = true
            continue
        }


    }
    if (!isPlaced) {
        for (let i = 0; i < intervals.length; i++) {
            const curInterval = intervals[i]
            const prevInterval = intervals[i - 1]
            const nextInterval = intervals[i + 1]
            if (curInterval[0] < newInterval[0] && (!nextInterval || nextInterval[0] > newInterval[0])) {
                if(curInterval[1] >= newInterval[1])break
                intervals.splice(i + 1, 0, newInterval)
                break
            }
            if (curInterval[0] > newInterval[0]  && (!prevInterval || prevInterval[0] < newInterval[0])) {
                intervals.splice(i, 0, newInterval)
                break
            }
        }
    }
    return intervals
};
```


