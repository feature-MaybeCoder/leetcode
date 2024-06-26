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
func balanceBST(root *TreeNode) *TreeNode {
	var dfs func(*TreeNode)
	tr := []int{}
	dfs = func(n *TreeNode) {
		if n == nil {
			return
		}
		dfs(n.Left)
		tr = append(tr, n.Val)
		dfs(n.Right)
	}
	dfs(root)
	return cons(tr)
}
func cons(tr []int) *TreeNode {
	if len(tr) == 0 {
		return nil
	}
	mid := (0 + len(tr) - 1) >> 1
	val := tr[mid]

	return &TreeNode{
		Val:   val,
		Left:  cons(tr[:mid]),
		Right: cons(tr[mid+1:]),
	}
}
