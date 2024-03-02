

----

Tags: #linkedList #medium #leetcode #repeat 

----

## Drawing:
[[reverse-linked-list-ii.excalidraw]]

----


## solution explanation:
#tip create dummy node for handling edge cases in linked list
solution: create dummy node, iterate until we reach left node
add two pointers, one to prev left second to left for the future 
iterate over reverting portion, set cur next pointer to prev node, prev node to cur, cur to next of current, in the end we will be end with two node, one is the right node (prev) and second is the next of right node (cur)
set left prev node next pointer to right node, and left node to next of right node


## last submission:
```javascript
class _ListNode {
    constructor(val, next) {

        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}
/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} left
 * @param {number} right
 * @return {ListNode}
 */
var reverseBetween = function (head, left, right) {
    const dummy = new _ListNode(0)
    dummy.next = head

    let prev = dummy, cur = head
    for (let i = 1; i < left; i++) {
        prev = cur
        cur = cur.next
    }
    let lp = prev, ls = cur
    let diff = right - left
    for (let i = 0; i <= diff; i++) {
        let tempNext = cur.next
        cur.next = prev
        prev = cur
        cur = tempNext
    }
    lp.next = prev
    ls.next = cur

    return dummy.next
};
```
