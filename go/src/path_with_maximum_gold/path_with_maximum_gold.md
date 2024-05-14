

----

Tags: #leetcode #matrix #depth-first-search

----

## Drawing:
[[path_with_maximum_gold.excalidraw]]

----
# Approach

if cur cell is not visited (!=0), run recursion, on each recursion step, mark cur cell as visited (cell=0 mutate inplace), find max path on connected non-visited neighbours, mark cell as non-visited, return cell value plus founded max

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(n)$$