#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut sentinel = ListNode::new(-1);
        let mut last_solution_node = &mut sentinel;
        let mut index = 1;
        
        while let Some(mut h) = head.take() {
            if index == k{
                for _ in 0..k {
                    
                }
            }
            index+=1;
            head = h.next.take();
        }
        // Perform reversal of k groups without counting.
        'finish: loop {
            for _ in 0..k {
                if let Some(mut current) = head.take() {
                    head = current.next.take();
                    current.next = last_solution_node.next.take();
                    last_solution_node.next = Some(current);
                } else {
                    break 'finish
                }
            }
            while let Some(ref mut next) = last_solution_node.next {
                last_solution_node = next;
            }
        };

        sentinel.next.take()
}

#[cfg(test)]
mod tests {
    use super::{reverse_k_group, ListNode};
    
    #[test]
    fn base_case() {
        assert_eq!(
            reverse_k_group(construct_list(vec![1, 2, 3, 4, 5]), 2),
            construct_list(vec![2, 1, 4, 3, 5])
        );
    }
}
