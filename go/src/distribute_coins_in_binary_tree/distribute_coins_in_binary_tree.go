package leetcode

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}
type DfsNode struct {
	coins int
	size  int
}

func max(num1 int, num2 int) int {
	if num1 > num2 {
		return num1
	}
	return num2
}

func distributeCoins(root *TreeNode) int {
	res := 0
	dfs(root, &res)
	return res
}
func abs(num1 int)int{
	if num1 < 0{
		return num1 *-1
	}
	return num1
}
func dfs(node *TreeNode, GCoins *int) DfsNode {
	if node == nil {
		return DfsNode{0, 0}
	}
	
	right := dfs(node.Right, GCoins)
	left := dfs(node.Left, GCoins)
	size := right.size + left.size + 1
	coins := node.Val + right.coins + left.coins
	*GCoins += abs(size - coins)
	return DfsNode{coins,size}
}
