

----

Tags: #leetcode #medium #dp #string

----

## Drawing:
[[interleaving_string.excalidraw]]

----


## solution explanation:
two possible solutions: 
first: basic dfs with binary decision tree and caching
second is dp wit matrix, each cell depends on rules: curent symbol by cell index is equal to main string, if so, check bottom or right cell, if so, set cell to true
