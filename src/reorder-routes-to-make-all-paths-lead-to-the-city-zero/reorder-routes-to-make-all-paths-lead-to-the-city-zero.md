


----

Tags: #graph #medium #leetcode #repeat #depth-first-search 

----

## Drawing:
[[reorder-routes-to-make-all-paths-lead-to-the-city-zero.excalidraw]]

----


## solution explanation:
#tip : represent DAG graph as connections with parallel edges from to and to from, (this need for iterating over ALL graph, not only reachable node from root), then use simple visited set for avoding loop iterations, route needs to be rotated if path we have path (from, to) not (to, from)
example: (1, 0) we start at 0 node, and move to 1, but we dont have (0,1) that means 0 is unreachable from 1

## last submission:
```javascript
/**
 * @param {number} n
 * @param {number[][]} connections
 * @return {number}
 */
var minReorder = function (n, connections) {
    const graph = Array.from({ length: n }, (_, i) => [])
    const pathes = new Set()
    const visited = new Set()
    const getHash = (from, to) => from + '' + '|' + to + ''
    for (const [from, to] of connections) {
        pathes.add(getHash(from, to))
        graph[from].push(to)
        graph[to].push(from)
    }
    let changeAmount = 0
    
    const dfs = (node) => {

        for (const neighbor of graph[node]) {
            if (visited.has(neighbor)) continue
            const pathHash = getHash(neighbor, node)
            
            if (!pathes.has(pathHash)) {
                changeAmount++
            }
            visited.add(neighbor)

            dfs(neighbor)
        }

    }
    visited.add(0)
    dfs(0)

    return changeAmount

};
```



