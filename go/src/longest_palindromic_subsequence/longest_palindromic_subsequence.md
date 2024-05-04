

----

Tags: #leetcode #medium #palindrom #string #dp #subsequence #lcs 

----

## Drawing:
[[longest_palindromic_subsequence.excalidraw]]

----


# Intuition

the subs palidrom len is amount of matched chars inside subs, for example most left and right chars are matched a and a, that means we found palidrom with at least two chars, if we have another characters inside this string, we need to check all of them in case of match, if we found another match that means our palidrom len is incremented by two, if inside this palindrom is no match but at least one char, that means we found odd palindrom with len 3

  

# Approach

we can solve this problem using top->down or bottom->up dp approach
if len of cur subs is len 1 -> return 1 (most shortest palindrom)
if chars is match, move both left and right pointer inside subs
if chars is unmatched find max from left and right

  

# Complexity

- Time complexity:

 $$On^2$$

  

- Space complexity:

$$On^2$$