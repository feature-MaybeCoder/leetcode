

----

Tags: #leetcode #medium #array #stack #string #sliding-window

----

## Drawing:
[[longest_repeating_character_replacement.excalidraw]]

----


## solution explanation:
we calculate needed to replace amount by subtracting from cur substr len most common char amount
#tip to remove iteration over all array for finding max of values at the end of current loop itter, we can create additional loop while needed condition is falsy, at the end we will get needed result
