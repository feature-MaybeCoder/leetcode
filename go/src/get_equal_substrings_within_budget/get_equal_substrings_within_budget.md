

----

Tags: #leetcode #subsequence #medium #string #sliding-window

----

## Drawing:
[[get_equal_substrings_within_budget.excalidraw]]

----


# Intuition

<!-- Describe your first thoughts on how to solve this problem. -->

  

# Approach

calculate cost for each char to replace, start sliding window over costs array, if cur window cost is greater then target, then we need to shrink left side of the window while sum of the window is greater or when we match right pointer

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(n)$$