

----

Tags: #leetcode #math #hard #parse

----

## Drawing:
[[integer_to_english_words.excalidraw]]

----


# Intuition

#tip to extract nths digit from number we can devide number by 10^nths then shift number to right by finding module

  

# Approach

split number to categories billions, millions, thousand, hundreds (untitled) apply rule to each category:
we can split category into subcategories, hundreds, tens, digit

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(n)$$