

----

Tags: #leetcode #medium #array #sliding-window

----

## Drawing:
[[number_of_sub_arrays_of_size_k_and_average_greater_than_or_equal_to_threshold.excalidraw]]

----


## solution explanation:
basic fixed sliding window problem, fill init sum by iterating to window threshold size, move window to right on each iteration step, keep track of current window sum by subtracting from sum most left element and add new right window, if current sum is ge than needed threshold, increment subarray num counter 
