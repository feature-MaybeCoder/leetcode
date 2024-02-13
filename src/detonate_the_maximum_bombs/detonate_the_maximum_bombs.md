

----

Tags: #leetcode #depth-first-search #graph

----

## Drawing:
[[detonate_the_maximum_bombs.excalidraw]]

----


## solution explanation:
break problem into two subproblems: create graph from input data and detecting is bomb inside another bomb and reverse, push bomb to parent
then traverse graph starting at each possible node
#tip union find is not working properly because graph needs to be undirected, each node can detonate parent and be dontanated by parent
