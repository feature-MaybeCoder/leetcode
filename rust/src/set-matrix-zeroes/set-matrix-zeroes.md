

----

Tags: #matrix #leetcode #medium

----

## Drawing:
[[set-matrix-zeroes.excalidraw]]

----


## solution explanation:
two solution: create visited map for all cells, iterate over matrix, run dfs on each zero cell
second solution: create need to replace row and col sets, iterate over matrix, if val is zero, add cur row and col to sets, iterate over matrix rows and cols, if cur row or col in row or col set acord, set all row or col to zero 

note: second solution is need n+k extra memory and 02n time complexity when first solution is needed n*k extra memory and 0n^2 time complexity



## last submission:
```javascript

```



