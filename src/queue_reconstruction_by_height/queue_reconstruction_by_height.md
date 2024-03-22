

----

Tags: #leetcode #medium #tree #segment-tree #priority-queue

----

## Drawing:
[[queue_reconstruction_by_height.excalidraw]]

----


## solution explanation:
can be solved using 3 solutions:
#### frist insert solution
sort input array by first item in desc order, if first item of cmp items is eq than cmp by pos in queue in asc order. sorting that way we will be shure that all elements after current item is having less or equal height, and sorting by position in queue of eq by height will give us knowledge: if we insert in res array by height amount of ge items before this item will match exact needed number. after sorting we insert into res array in index of number items before current item with shift to the left
O notation:
On^2 rc
O1 sc (excluding output)
### second BIT recursion solution:
sort input array by height in asc order and by pos in queue in desc order
construct BIT tree where each leaf will repsent remaining space in the result array
iterate over items and start binary search in bit tree with rules:
check remaining space in left segment, if space in the left is greater than item position in the queue go to the left, else got to the right and decrement value by the remaining space on the left, iterate that way in the tree and decrement remaining space in segment on each step of the recursion, in the end when left and right pointers will match we willl get item position in the reconstructed queue
Onlogn rc
Onlogn sc (tree)
### third BIT loop solution:
this solution is pretty similar to second, but we use another technique to store BIT tree, that will require On sc:
sort input array the same way, init bit tree array with len of input +1,
