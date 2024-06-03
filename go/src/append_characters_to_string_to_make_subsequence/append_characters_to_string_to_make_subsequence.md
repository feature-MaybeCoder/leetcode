

----

Tags: #leetcode #medium #subsequence #string #lcs #two-pointers

----

## Drawing:
[[append_characters_to_string_to_make_subsequence.excalidraw]]

----


# Intuition

<!-- Describe your first thoughts on how to solve this problem. -->

  

# Approach

we can solve this problem in two ways: first is find lcs of two strings, then subtract from t len founded lcs , this solution have O s * t  time complexity and memory complexity.
else we can solve this problem using simple two pointers solution, if chars matches we move both pointers forward, else we move s pointer to the right, in the end subtract t pointer from t len

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(1)$$