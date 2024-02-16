function rev(head, start,tail, amount){
    for (let i =0; i < amount; i++){
        
        let temp = start.next
        start.next = tail;
        tail = start;
        start = temp;
    }
    
    if (head){
        head.next = tail
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
 * @param {number} k
 * @return {ListNode}
 */
var reverseKGroup = function(head, k) {
    let head_r = null
    let prev = null
    let start = head
    let i =1
    while(head){
        if (i === k){
            rev(prev, start, head.next, k)
            prev = start
            start = start.next
            if(!head_r){
                head_r = head
            }
            head = start
            i = 1
            continue
        }
        head = head.next
        i++
    }
    return head_r
};
