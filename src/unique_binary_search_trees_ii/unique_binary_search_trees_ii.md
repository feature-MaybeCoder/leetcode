

----

Tags: #leetcode #medium #dp #binary-tree

----

## Drawing:
[[unique_binary_search_trees_ii.excalidraw]]

----


## solution explanation:
start recursion at each possible head, move left recursion
to from::index-1
move right recursion to to::index+1
for each possible left and right subtree generate new head node and push it to res
