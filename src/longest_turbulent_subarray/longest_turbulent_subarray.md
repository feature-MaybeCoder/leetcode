

----

Tags: #leetcode #medium #array

----

## Drawing:
[[longest_turbulent_subarray.excalidraw]]

----


## solution explanation:
simple sliding window technique, set two pointers, left and right, save prev number of right pointer and prev cmp operation value, if cur compare with prev number lead us to equal or to the same operation as prev we need to shift left pointer and set our res variable to cur diff between right and left pointers
