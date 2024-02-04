

----

Tags: #leetcode #graph #hard #string #union-find

----

## Drawing:
[[similar_string_groups.excalidraw]]

----


## solution explanation:
recreate union find structure for each possible combination, that lead us to On^2 time complexity
#tip to count unique islands in union find we don't need additional loop in the end, simply init island count at length of nodes and decrement this count for each completed merge