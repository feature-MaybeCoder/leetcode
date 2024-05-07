package double_a_number_represented_as_a_linked_list

type ListNode struct {
	Val  int
	Next *ListNode
}
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
 func doubleIt(head *ListNode) *ListNode {
	if head !=nil && head.Val > 4{
          head = &ListNode{
              Val: 0,
              Next: head,
          }
      }
	for cur := head; cur != nil; cur = cur.Next{
		cur.Val = cur.Val *2 %10
		if cur.Next != nil && cur.Next.Val > 4{
			cur.Val +=1
		}
     }
	return head
  }
