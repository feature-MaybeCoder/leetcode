

----

Tags: #leetcode #matrix #depth-first-search #breath-first-search #hard

----

## Drawing:
[[minimum_number_of_days_to_disconnect_island.excalidraw]]

----


# Intuition

main intuition is that you already have disconnected grid, so you need to make 0 moves
or you can disconnect grid in one move
or you can disconnect grid in at most two moves THATS ALL
because you can disconnect any border cell in a grid by remove two diagonal cell see drawing

  

# Approach

init amount helper function that will run dfs and count islands 
run init check on islands, if we have not one island, then we don't need to make any moves, return 0
iterate over the matrix and extract cell, check if extracting this cell made matrix disconnected, if so we made it in one move
if there is no possible one move combination, then we need to make two moves
#tip to not create helper visited matrix on every amount fc call, we can use swap trick:
get two variable, target and markTo,  check is cell a target, if no, cell is already visited or it is a water, and mark it as markTo,
in the end of iteration over a matrix each land cell will be markTo, so we can simply swap this variables for next iteration 

there is actual runtime downside: we can't use early break when we reach more then one island, we need to mark all the land cells, if we don't mark all the cells, on next iter step we get currpted data with both target and markTo values as land

# Complexity

- Time complexity:

 $$O((h*w)^2)$$

  

- Space complexity:

$$O(n)$$ only for recursion stack