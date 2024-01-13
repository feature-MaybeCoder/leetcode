

----

Tags: #graph #leetcode #medium #union-find

----

## Drawing:
[[minimum-number-of-vertices-to-reach-all-nodes.excalidraw]]

----


## solution explanation:
two solutions: first complex and inoficiant: union find, then return all unique parents
second solution: what puzzle is about is returning all roots, because root itself must be in reses array because we cannot visit itself from any another node. solution: iterating over all edges, and set TO node to true in set, that means that this node is not root, find all nodes that not in the set, that nodes are roots and we need to add them into res arr

## last submission:
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findSmallestSetOfVertices = function (n, edges) {
    const parrents = Array.from({ length: n }, (_, i) => false)

    for (const [from, to] of edges) {
        parrents[to] = true
    }

    const reses = []
    for (let i = 0; i < n; i++) {
        if (parrents[i]) continue
        reses.push(i)
    }
    return reses
};
```
## last union find submission:
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findSmallestSetOfVertices = function (n, edges) {
    const parrents = Array.from({ length: n }, (_, i) => i)
    
    const find = (node) => {
        let p = parrents[node]
        while (p !== parrents[p]) {
            p = parrents[p] = parrents[parrents[p]]
        }
        return p
    }
    const union = (from, to) => {
        const pTo = find(to)
        if (pTo !== to) return
        const pFrom = find(from)
        parrents[pTo] = pFrom
    }
    for (const [from, to] of edges) {
        union(from, to)
    }
    const visited = new Set()
    const reses = []
    for (let i = 0; i < n; i++) {
        const p = find(i)
        if (visited.has(p)) continue
        visited.add(p)
        reses.push(p)
    }
    return reses
};
```


