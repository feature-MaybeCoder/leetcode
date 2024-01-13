

----

Tags: #graph #medium #leetcode #dp

----

## Drawing:
[[is-graph-bipartite.excalidraw]]

----


## solution explanation:
run dfs on each possible graph as started node be in group a, we come from node that are in group a, and current node, in group a than graph is not  **bipartite** else  **bipartite**

## last submission:
```javascript
/**
 * @param {number[][]} graph
 * @return {boolean}
 */
var isBipartite = function (graph) {
    const groupASet = new Set()
    const groupBSet = new Set()

    const len = graph.length
    const dfs = (node, isFromA, from) => {
        
        if (isFromA && groupASet.has(node) || !isFromA && groupBSet.has(node)) {
            return false
        }
        if (groupASet.has(node) || groupBSet.has(node)) {
            return true
        }
        if (isFromA) {
            groupBSet.add(node)
        } else {
            groupASet.add(node)
        }
        for (const connection of graph[node]) {
            if (connection === from) continue
            const is = dfs(connection, !isFromA, node)
            if (!is) return false
        }
        return true

    }
    for (let node = 0; node < len; node++) {
        if (groupASet.has(node) || groupBSet.has(node)) {
            continue
        }
        const is = dfs(node, false)
        if (!is) return false
    }
    return true
};
```



