

----

Tags: #leetcode #sliding-window #medium

----

## Drawing:
[[best-time-to-buy-and-sell-stock-ii.excalidraw]]

----


## solution explanation:
iterate from right to left, find first up trend, add to diff to profit, update max, repeat process 

## last submission:
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function (prices) {
    const _len = prices.length - 1

    let right = _len
    let profit = 0
    let curMax = prices[_len]
    while (right > 0) {
        let curMin = curMax
        let i = right - 1
        while (i >= 0) {
            let cur = prices[i]
            if (cur > curMin) {
                break
            }
            curMin = cur
            i--
        }
        profit += curMax - curMin
        curMax = prices[i]
        right = i
    }
    return profit
};
```



