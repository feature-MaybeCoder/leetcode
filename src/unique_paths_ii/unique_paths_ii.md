

----

Tags: #dp #leetcode #medium #matrix #depth-first-search

----

## Drawing:
[[unique_paths_ii.excalidraw]]

----


## solution explanation:
basic dp problem, start from bottom to up, calculate amount of paths with rules: current cell have sum of right and bottom cells unique pathes, if cell is obstacle than it has 0 pathes, replace 1 with 0 (0 represent obstacle) and calculate paths in runtime #todo: remove rudiment if statement for first cell, that will improve runtime
