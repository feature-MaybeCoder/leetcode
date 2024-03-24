

----

Tags: #leetcode #medium #binary-tree #segment-tree

----

## Drawing:
[[my_calendar_i.excalidraw]]

----


## solution explanation:
keep track of added boks in sorted array, on new book start binary search on an array, on each step check the intersection, on last elemnt check the intersecion, if there is no intersection add new book to forward or backward base on cmp of first item
### notation
On sc
O tc because of insertion with shift to the left
