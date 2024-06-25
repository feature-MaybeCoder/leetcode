package leetcode

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func bstToGst(root *TreeNode) *TreeNode {
	node, _ := dfs(root, 0)
	return node
}
func dfs(node *TreeNode, sum int) (*TreeNode, int) {
	if node == nil {
		return nil, 0
	}
	right, rSum := dfs(node.Right, sum)
	curSum := node.Val + sum + rSum
	left, lSum := dfs(node.Left, curSum)

	return &TreeNode{
		Val:   curSum,
		Right: right,
		Left:  left,
	}, rSum + node.Val + lSum
}
