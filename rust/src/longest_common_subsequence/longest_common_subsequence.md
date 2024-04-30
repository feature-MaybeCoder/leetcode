

----

Tags: #leetcode #dp #medium

----

## Drawing:
[[longest_common_subsequence.excalidraw]]

----


## solution explanation:
### solution 1:
run recursion over string with cache by index:
if left idx and right idx in cache -> return cache value
if char in strings by cur indexes is match, move both indexes to +1, else find max from left +1, right and left, right +1 decision tree
### dp solution:
init 2d dp cache with left.len * right.len
iterate bottom to up, if cur x y chars is match, set cur cell to l