

----

Tags: #graph #medium #leetcode

----

## Drawing:
[[flower-planting-with-no-adjacent.excalidraw]]

----


## solution explanation:
there is no graph bypass algo needed, just iterating over all nodes, and set it type that neighbors of this node dosent have

## last submission:
```javascript
/**
 * @param {number} n
 * @param {number[][]} paths
 * @return {number[]}
 */
var gardenNoAdj = function (n, paths) {
    if (n < 5) {
        return Array.from({ length: n }, (_, i) => i + 1)
    }
    const graph = Array.from({ length: n }, (_, i) => [])
    const flowersPath = Array.from({ length: n }, (_, i) => null)

    for (let [from, to] of paths) {
        from -= 1
        to -= 1
        graph[from].push(to)
        graph[to].push(from)
    }
    const typeSet = new Set()
    const getNewType = (node) => {
        for (let parrent of graph[node]) {
            if (flowersPath[parrent] === null) {
                continue
            }
            typeSet.add(flowersPath[parrent])
        }
        for (let i = 1; i < 5; i++) {
            if (!typeSet.has(i)) {
                typeSet.clear()
                return i
            }
        }
    }
    for (let i = 0; i < n; i++) {
        flowersPath[i] = getNewType(i)

    }


    return flowersPath
};
```



