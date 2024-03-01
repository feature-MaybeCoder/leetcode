

----

Tags: #leetcode #medium #linkedList

----

## Drawing:
[[odd-even-linked-list.excalidraw]]

----


## solution explanation:
has multiple solutions, most elegant:
keep track of odd (head) and event (head.next)
get odd.next (even node) to temp var
set odd.next to odd.next.next (next odd node)
set temp var.next to odd.next.next (next even node)
set cur odd node to odd.next (next odd node)
repeat the process, in the end we will have pointer to the end of odd list and head of even list, set even tail.next to odd head, that will be our result
