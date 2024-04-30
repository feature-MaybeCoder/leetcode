

----

Tags: #leetcode #medium #linkedList #binary-tree

----

## Drawing:
[[convert_sorted_list_to_binary_search_tree.excalidraw]]

----


## solution explanation:
first solution: iterate over given list and collect values into vec , then create bts from vec recursively slicing given vec into two sesparate pieces one for left, one for right subtree, repeat this process while we have length of slice more than 0
second solution: count nodes amount in given list, start recursive function that will accept head and cur length, divide len by two and rollout to the left while we have some length, when we reach the end, we need to create current node, and shift ref of the head to the next position. rollout to the right if possible (right len is len - mid -1), using that technique we always be in the needed spot for our current subtree
#tip move head pointer by ref from the leaf of the recursion 