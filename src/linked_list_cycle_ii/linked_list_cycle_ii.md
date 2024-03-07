

----

Tags: #leetcode #medium #linkedList #repeat

----

## Drawing:
[[linked_list_cycle_ii.excalidraw]]

----


## solution explanation:
keep track of two pointers fast and slow, move slow to the right by one and fast by two on each iteration step, if we reach the position where slow and fast is equal thats mean we detect the cycle, to find the node that lead us to cycle we need to reset one of the pointers to the head of the linked list we given and move each pointer by one, when we pointers are meets we need to return cur pointer node