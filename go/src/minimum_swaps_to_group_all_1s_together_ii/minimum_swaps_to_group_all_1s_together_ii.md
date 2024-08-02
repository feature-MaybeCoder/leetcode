

----

Tags: #leetcode #medium #array #sliding-window

----

## Drawing:
[[minimum_swaps_to_group_all_1s_together_ii.excalidraw]]

----


# Intuition

amount of swaps is amount of zeros in subarray
we need to check all possible subarrays including circulars
to avoid recalculating amount of zeros we can use sliding window approach

  

# Approach
count amount of ones in subarray, that will be the len of subarray to check
init subarray of len n, count zeros in this subarray
for each iter step, subtract from zeros amount if element to the left is zero, add if new element to the right is zero
update minimum
after we check all possible subarrays we need to check circular subarrays
move right p to left position,
move left to start of the array
repeat the process

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(1)$$