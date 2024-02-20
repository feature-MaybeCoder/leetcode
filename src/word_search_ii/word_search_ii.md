

----

Tags: #leetcode #hard #matrix #depth-first-search #tree

----

## Drawing:
[[word_search_ii.excalidraw]]

----


## solution explanation:
build prefix tree, run dfs and shift node to next, if cur node is contains some end, push end to res 
#tip: use array as hashmap known laters range
#tip: count childrens of word search node structure, substract when we found end, break from recursions when we reach zero childrens
