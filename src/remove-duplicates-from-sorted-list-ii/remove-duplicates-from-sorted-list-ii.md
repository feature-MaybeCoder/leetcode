

----

Tags: #linkedList #medium #leetcode

----

## Drawing:
[[remove-duplicates-from-sorted-list-ii.excalidraw]]

----


## solution explanation:
two solutions:
colect non duplicates node in array, iterating over array and set nodes pointers to next item in arr or null
second solution: solve with three pointers, prev, next and next.next, if next not equal to next.next set prev.next to next and move next, else, iterating while we reach nonequal node, set prev next to nonequal node and repeat process

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
 * @return {ListNode}
 */
var deleteDuplicates = function (head) {

    let dummy = new _ListNode(null, head)
    let prev = dummy
    let isEqual = false
    while (head?.next) {
        if (head.next.val === head.val) {
            head = head.next
            isEqual = true
            continue
        }
        if (isEqual) {
            prev.next = head.next
            head = head.next
            isEqual = false
            continue
        }
        prev = head
        head = head.next

    }
    if (isEqual) {
        prev.next = head?.next || null
    }
    return dummy.next

};
```



