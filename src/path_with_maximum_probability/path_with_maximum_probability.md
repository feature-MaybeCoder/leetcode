

----

Tags: #leetcode #medium #graph #dijkstra

----

## Drawing:
[[path_with_maximum_probability.excalidraw]]

----


## solution explanation:
basic dijkstra algo: if node is already visited, that means we visit this node with ge probability, simply continue, else push all neighbors nodes of cur node to binary stack with prob * cur node prob
#tip keep track of node max/min so we can optimize dijkstra and do not push to stack value that already in stack/were on stack with ge/le value