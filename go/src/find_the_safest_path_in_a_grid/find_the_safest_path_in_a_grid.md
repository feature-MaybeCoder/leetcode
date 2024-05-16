

----

Tags: #leetcode #medium #matrix #breath-first-search #depth-first-search #dijkstra #manhattan 

----

## Drawing:
[[find_the_safest_path_in_a_grid.excalidraw]]

----


# Intuition

use bfs for range calculation, use dijkstra for finding most efficient path

  

# Approach

first stage of solution is to most optimal calculate manhattan distance for each to cell to nearest thief, we can do it by iterating over all matrix, acum all thief cells, start bfs with multiple starts at all thief cell, if current cell having smaller or equal mh dist, we dont need to iterating over it again, simply continue bfs loop, in the end we will get new matrix with mh evaluation in $O(n)$ time complexity
second stage if dijkstra algorithm for finding most optiomal path, but while iterating over heap we don't need to summup, we need to keep track of minimum value in a path

  

# Complexity

- Time complexity:

 $$O(N^2logN)$$

  

- Space complexity:

$$O(n^2)$$