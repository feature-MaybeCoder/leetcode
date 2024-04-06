

----

Tags: #leetcode #medium #graph #prims

----

## Drawing:
[[min_cost_to_connect_all_points.excalidraw]]

----


## solution explanation:
can be solved in different ways, more intuitive is primrims algo: start from 0 point, populate all possible points to min heap, set point as visited, from this point populate all non visited points, pop min from min heap, repeat process wile we don't visit all points, very simillar do #dijkstra algo
requeres On sc and On * n logn tc where n is edges + points
