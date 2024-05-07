package double_a_number_represented_as_a_linked_list

type ListNode struct {
	Val  int
	Next *ListNode
}
func dfs(head *ListNode) int {
	if head == nil{
		return 0
	}

	new_val := head.Val *2 + dfs(head.Next)
	if new_val >= 10{
		ones := 0
		for new_val >= 10{
			ones += 1
			new_val -= 10
		}
		head.Val = new_val
		return ones
	}
	head.Val = new_val
	return 0
}
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func doubleIt(head *ListNode) *ListNode {
	ret := dfs(head)
	if ret > 0 {
		return &ListNode{
			Val: ret,
			Next: head,
		}
	}
	return head
}
