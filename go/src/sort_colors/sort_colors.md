

----

Tags: #leetcode #array #sorting #two-pointers #repeat

----

## Drawing:
[[sort_colors.excalidraw]]

----


# Intuition

<!-- Describe your first thoughts on how to solve this problem. -->

  

# Approach

we can easily solve this problem using count hashmap, that would lead us to linear tc and mc, but we can solve this in constant mc, using [counting sort grouping](https://en.wikipedia.org/wiki/Dutch_national_flag_problem)start with three pointers, left, mid, right, mid is always pointing to the end (where we need to insert 2), if mid value is 0 that means we need to swap it back, swap left and right, move both pointers to the right, if mid value is 1, that means 1 is in place (or we will swap it when we found 0), move mid to the right, else value is 2, so we move it to the right, move right pointer to the left so next inserted 2 will be on the left

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(1)$$