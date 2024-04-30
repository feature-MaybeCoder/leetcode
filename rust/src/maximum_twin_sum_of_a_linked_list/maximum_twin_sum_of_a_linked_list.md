

----

Tags: #leetcode #medium #linkedList #repeat

----

## Drawing:
[[maximum_twin_sum_of_a_linked_list.excalidraw]]

----


## solution explanation:
create two pointers, one to the head, second to the tail, unwrap recursion till the end of the list, sumup head value by pointer and current tail node, move head pointer to the right, rollout from recursion
if next head pointer is equals to the current node that means our pointers is meets in the middle and we need to exit from recursion
### optimizations room:
we can write a much simpler solution by simply using vec to store all of the nodes values
### complexity
On runtime
On memory for stack
