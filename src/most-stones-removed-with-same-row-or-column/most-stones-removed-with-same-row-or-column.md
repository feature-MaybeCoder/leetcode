

----

Tags: #graph #leetcode #medium #union-find #repeat 

----

## Drawing:
[[most-stones-removed-with-same-row-or-column.excalidraw]]

----


## solution explanation:
union find conception: init two list with parent and weights, each node belong to itself and have init weight of 1, find parents of two nodes and union them with this rules: change parent root of node that have less weight, if nodes have the same parrent union cant be completed. in this puzzle after union by x and y  coordinates we can calculate how many removed stones we have

## last submission:
```javascript
/**
 * @param {number[][]} stones
 * @return {number}
 */
var removeStones = function (stones) {
    const parents = Array.from({ length: stones.length }, (_, i) => i)
    const weights = Array.from({ length: stones.length }, (_, i) => 1)
    let xMap = new Map()
    let yMap = new Map()
    let removed = 0
    const find = (i) => {
        let p = parents[i]
        while (p !== parents[p]) {
            p = parents[p] = parents[parents[p]]

        }
        return p
    }
    const union = (x, y) => {
        const parX = find(x)
        const parY = find(y)
        if (parX === parY) return false
        if (weights[parX] > weights[parY]) {
            parents[parY] = parX
            weights[parX] = weights[parX] + weights[parY]
            return true
        }
        parents[parX] = parY
        weights[parY] = weights[parY] + weights[parX]
        return true
    }
    for (let i = 0; i < stones.length; i++) {

        const [x, y] = stones[i]
        let hasX = xMap.has(x)
        let hasY = yMap.has(y)
        if (hasX) {
            let isSucsses = union(i, xMap.get(x))
            if (isSucsses) {
                removed++
            }

        } else {
            xMap.set(x, i)
        }
        if (hasY) {
            let isSucsses = union(i, yMap.get(y))
            if (isSucsses) {
                removed++
            }
        } else {
            yMap.set(y, i)
        }


    }
    return removed
};
```



