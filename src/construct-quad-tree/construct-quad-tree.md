

----

Tags: #leetcode #medium #tree

----

## Drawing:
[[construct-quad-tree.excalidraw]]

----


## solution explanation:
two solutions:
iterate over all matrix, if found non equal values, break matrix into four submatrises, repeat process, time complexiy: 0n^2 * log N

efective solution:
create node, immediately break matrix untill we reach one cell, when recursion is roll out, check created nodes, if all fore nodes is leafs and have the same value, that nodes is unnided, return current node as leaf without created nodes, repeat process
timeComplexity: n * n

## last submission:
```javascript

```



