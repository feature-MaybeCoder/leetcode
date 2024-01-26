

----

Tags: #graph #hard #leetcode #breath-first-search

----

## Drawing:
[[word_ladder.excalidraw]]

----


## solution explanation:
main time complexity is in traversing input string to graph representations, that lead us to On^2 complexity, than we need just basic bfs traversal of a graph, with first match of last index return cur iteration depth, if we didnt reach last index after all bfs traversal return 0
