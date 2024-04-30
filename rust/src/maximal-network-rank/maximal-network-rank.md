

----

Tags: #leetcode #graph #medium

----

## Drawing:
[[maximal-network-rank.excalidraw]]

----


## solution explanation:
#tip find min and max for uniform index for manage pair visited. iterate over all edges, incriment visited amount of both nodes, iterate for each node, to all possible pair (n^2) find max of curent and sum of weights of two nodes (decrement by one if pair in set)

## last submission:
```javascript
/**
 * @param {number} n
 * @param {number[][]} roads
 * @return {number}
 */
var maximalNetworkRank = function (n, roads) {
    const visited = Array.from({ length: n }, () => 0)
    const pairsSet = new Set()
    const getIndex = (from, to) => {
        return Math.min(from, to) + "|" + Math.max(from, to)
    }
    for (const [from, to] of roads) {
        visited[from]++
        visited[to]++
        pairsSet.add(getIndex(from, to))
    }
    let max = 0
    
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
            if (pairsSet.has(getIndex(i, j))) {
                max = Math.max(visited[i] + visited[j] - 1, max)
                continue
            }
            max = Math.max(visited[i] + visited[j], max)
        }
    }
    return max
};
```



