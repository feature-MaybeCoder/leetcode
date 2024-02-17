

----

Tags: #leetcode #hard #binary-tree #graph

----

## Drawing:
[[binary_tree_maximum_path_sum.excalidraw]]

----


## solution explanation:
run postorder traversal on BT, update max by sum of cur node value + sum of left and right if left and right is ge than zero, return max possible pass node.val + left or node.val + right
