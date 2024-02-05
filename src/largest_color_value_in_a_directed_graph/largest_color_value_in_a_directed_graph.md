

----

Tags: #leetcode #hard #graph #dp #2d_cache #cache #depth-first-search #topological-sort

----

## Drawing:
[[largest_color_value_in_a_directed_graph.excalidraw]]

----


## solution explanation:
could be solved with dfs or topological sort
dfs solution: main problem is to add caching while traversing graph, manage loops with default dfs technique, while entering new layer of recursion if node is in stack that means we reach loop and we need to return -1, else if we reach node that we visited earlier, return max value from cache (that needs for main loop), in dfs function traverse all the children, after returning from the child we needed to add caches with rules: insert in cur node cache max of current cached value and children value or if cur node has the same color children value + 1, return from add function max value, update in dfs loop max value,
#tip if we know amount of unique items, then 2d cache is not that scary for complexity
#todo: solve with tp sort
