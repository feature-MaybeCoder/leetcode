

----

Tags: #leetcode #array #medium #heap #repeat

----

## Drawing:
[[longest-increasing-subsequence.excalidraw]]

----


## solution explanation:
three possible solutions:
### dfs
first is dfs with decision tree of width of array length, without caching this solution is required 2^n time complexity, cache is simplify runtime to n^2 with 0n memory complexity for cache. logic: start at each index, if we reach this index earlier return cached value, else iterating over index..len if value is greater run dfs for it, find max of all dfses for this index, add to cache max+1 and return this value
### dp
start at end of the array, create depth lookup array, for each index, start iterating index..end and find maximum of calculated depths for previous indexes, when value at this index is greater than current value On^2 runtime plus 0n memory
## dp + sorted lookup array
create lookup array with first value, iterate over all nums and cmp with last added value into lookup array, if value is greater than last, simply push, else if val is less, then run binary search to this value, if there is no such value, get last piwot and cmp current value to piwot, if piwot is greater, then change piwot to current value


## last submission:
```javascript

```



