

----

Tags: #leetcode #medium #string #dp #palindrom

----

## Drawing:
[[palindromic_substrings.excalidraw]]

----

# Intuition

check is substr is palindrom expanding from center of substr

  

# Approach

for each index in str expand towards end of str two times, event and odd, increment counter on each iteration step (each iteration step means one palindrom)

  

# Complexity

- Time complexity:

 $$O(n)^2$$

  

- Space complexity:

$$O(1)$$