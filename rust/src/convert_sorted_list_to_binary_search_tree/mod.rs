use std::cell::RefCell;
use std::rc::Rc;

use crate::test_helpers::{ListNode, TreeNode};
fn construct_bts_from_head(
    head: &mut Option<Box<ListNode>>,
    len: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    
    if len == 0 {
        return None;
    }
    let mid = len /2;
    let left = construct_bts_from_head(head, mid);
    let node = head.take().unwrap();
    *head = node.next;
    let right = construct_bts_from_head(head, len - mid  -1 );
    let mut bts_node = TreeNode::new(node.val);
    bts_node.left = left;
    bts_node.right = right;

    Some(Rc::new(RefCell::new(bts_node)))
}
pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut len = 0;
    let mut head_ref = head.as_ref();
    while let Some(head_node) = head_ref {
        len += 1;
        head_ref = head_node.next.as_ref();
    }
    construct_bts_from_head(&mut head, len)
}
fn construct_bts_from_vec(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if slice.len() == 0 {
        return None;
    }
    let mid = slice.len() >> 1;
    let mut node = TreeNode::new(slice[mid]);
    node.right = construct_bts_from_vec(&slice[mid + 1..slice.len()]);
    node.left = construct_bts_from_vec(&slice[..mid]);

    Some(Rc::new(RefCell::new(node)))
}
pub fn sorted_list_to_bst_vec(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut values: Vec<i32> = Vec::new();
    let mut head_ref = head.as_ref();
    while let Some(head_node) = head_ref {
        values.push(head_node.val);
        head_ref = head_node.next.as_ref();
    }
    construct_bts_from_vec(&values[..])
}
#[cfg(test)]
mod test {
    use crate::test_helpers::{construct_bt_from_traversal, construct_list};

    use super::sorted_list_to_bst;

    #[test]
    fn base_case() {
        let bts = construct_bt_from_traversal(
            0,
            &vec![Some(2), Some(1), None, None, Some(3), None, None],
        );
        assert_eq!(sorted_list_to_bst(construct_list(vec![1, 2, 3])), bts.1);
    }
    #[test]
    fn one_left_node_case() {
        let bts = construct_bt_from_traversal(
            0,
            &vec![Some(3), Some(1), None, None, None, None],
        );
        assert_eq!(sorted_list_to_bst(construct_list(vec![1,3])), bts.1);
    }
}
