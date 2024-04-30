

----

Tags: #leetcode #medium #bits

----

## Drawing:
[[bitwise_and_of_numbers_range.excalidraw]]

----


## solution explanation:
first solution: shift left and right ints to the right by one while left and right is not equal, increment counter, in the end shift left to the left by counter 
because bits Not equality is always in right side of integer, if ints is not equal we need to shift to right in order to get rid of not equal bits, if ints is equals, that means we reach left side of int that is equal and wee need to shift it to the left for restoring previous size

second solution: for 0..32 calculate in what interval current bit can change 
10
^
|
changes every second number for example
calculate diff right - left and shift it by amount of one bits of left number to the right of curent bit if diff is less than interval that means that byte could be possible 1, bit is 1 if that bit is 1 in left and right number