

----

Tags: #graph #medium #leetcode #dp #repeat 

----

## Drawing:
[[minimum-height-trees.excalidraw]]

----


## solution explanation:
found two solution: implement solution with two algos: dfs (incorrect), bfs (correct),
traverse graph into more helpful representation, found all leaves, grow selection from leaves to parents, add parents that became leaves
## performance notes:
matrix of connection actually more performant than object of object with complexity of needed operations o(1)
## last submission:
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findMinHeightTrees = function (n, edges) {
    if (n < 2) return [0]
    const graph = Array.from({ length: n }, () => []);


    for (const edge of edges) {
        const node1 = edge[0]
        const node2 = edge[1]
        graph[node1].push(node2)
        graph[node2].push(node1)
        
    }
    let returnLeaf = []
    for (let i = 0; i < n; i++) {
        const len = graph[i].length
        if (len === 1) {
            returnLeaf.push(i)
        }
    }
    while (n > 2) {

        const tempReturnLeaf = []
        for (const leaf of returnLeaf) {
            const connection = graph[leaf].pop()
            const arr = graph[connection]
            arr.splice(arr.indexOf(leaf), 1)
            if (arr.length === 1) {
                tempReturnLeaf.push(connection)
            }
            n--
        }
        returnLeaf = tempReturnLeaf
    }
    return returnLeaf

};
```



