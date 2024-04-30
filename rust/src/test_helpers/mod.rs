// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub fn construct_list(mut vec: Vec<i32>) -> Option<Box<ListNode>> {
    vec.reverse();
    let mut head = None;
    for item in vec {
        let mut node = ListNode::new(item);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn construct_bt_from_traversal(
    index: usize,
    traversal: &Vec<Option<i32>>,
) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
    if let Some(val) = traversal[index] {
        let mut node = TreeNode::new(val);
        let (left_index, left_node) = construct_bt_from_traversal(index + 1, traversal);
        let (right_index, right_node) =
            construct_bt_from_traversal(left_index + 1, traversal);

        node.left = left_node;
        node.right = right_node;

        return (right_index, Some(Rc::new(RefCell::new(node))));
    }
    return (index, None);
}
