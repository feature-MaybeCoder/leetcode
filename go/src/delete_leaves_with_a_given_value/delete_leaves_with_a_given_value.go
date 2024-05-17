package leetcode
 type TreeNode struct {
     Val int
     Left *TreeNode
     Right *TreeNode
 }
func removeLeafNodes(root *TreeNode, target int) *TreeNode {
    if (root == nil){
        return nil
    }
    right := removeLeafNodes(root.Right, target)
    left := removeLeafNodes(root.Left, target)
    if right == nil && left == nil && root.Val == target {
        return nil
    }
    root.Right = right
    root.Left = left
    return root
}
