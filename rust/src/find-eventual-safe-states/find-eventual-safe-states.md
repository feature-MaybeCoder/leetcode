

----

Tags: #graph #medium #leetcode #breath-first-search #depth-first-search

----

## Drawing:
[[find-eventual-safe-states.excalidraw]]

----


## solution explanation:
trick: if we need sorted graph results, insert item in array not in recursion function, but in for loop where we iterating over each graph node
## last submission:
```javascript
/**
 * @param {number[][]} graph
 * @return {number[]}
 */
var eventualSafeNodes = function (graph) {
  const visitedMap = {}
  


  const dfs = (node) => {

    if (node in visitedMap) {
      return visitedMap[node]
    }
    const neighs = graph[node]
    if (!neighs?.length) {

      visitedMap[node] = true
      return true
    }
    visitedMap[node] = false
    let isAllSafe = true

    for (let i = 0; i < neighs.length; i++) {
      const neigh = neighs[i]
      const isNeighSafe = dfs(neigh)
      if (isAllSafe) {
        isAllSafe = isNeighSafe
      }
    }
    visitedMap[node] = isAllSafe
    return isAllSafe
  }
  const res = []
  for (let i = 0; i < graph.length; i++) {
    const isSafe = dfs(i)
    if (isSafe) {
      res.push(i)
    }
  }


  return res
};
```



