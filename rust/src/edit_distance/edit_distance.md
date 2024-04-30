

----

Tags: #dp #leetcode #medium #string #repeat

----

## Drawing:
[[edit_distance.excalidraw]]

----


## solution explanation:
### dfs:
two possible solutions: dfs with rules: start at end of each string, if some string len is 0, return max len, if each str len is 0 return 0,
if cur chars of strings is equals return next iteration of dfs and move two pointers, else find min of replace, del, insert
###### replace:
move both pointers
###### del:
move first pointer
###### insert:
move second pointer

we can add cache where key is tuple of two string pointers

### dp:
create matrix with len1+1 * len2+1
iterate over borders of each string, and set cell to len of string - index (border index cost of replacing all remaining)
then iterate over matrix from bottom to top, filling cells with rules:

###### equals:
set cell value as value that diagonally below right and continue

else set cell to min of bottom, right and diagonally below right cell  + 1 