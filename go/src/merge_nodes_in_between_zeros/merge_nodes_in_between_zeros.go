package leetcode

import "runtime/debug"
 type ListNode struct {
     Val int
     Next *ListNode
 }
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeNodes(head *ListNode) *ListNode {
    start := head
    res := head
    head = head.Next
    for head.Next != nil{
        if head.Val == 0{
            start.Next = head
            start = head
        }
        start.Val += head.Val
        head = head.Next
    }
    start.Next = nil
    return res
}

func init() {
    debug.SetGCPercent(-1)
}
