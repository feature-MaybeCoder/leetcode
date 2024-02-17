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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::MIN;
    dfs(&root, &mut max);
    return max
}
fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32)->i32{
    if let Some(node) = node {
        let node = node.borrow();
        let left = dfs(&node.left, max);
        let right = dfs(&node.right, max);
        let mut cur_res = node.val;
        if left > 0{
            cur_res+=left;
        }
        if right>0{
            cur_res+=right;
        }
        if cur_res > *max {
            *max = cur_res;
        }
        return node.val + left.max(right).max(0)
    }
    return 0
}
#[cfg(test)]
mod test{
    use std::{cell::RefCell, rc::Rc};

    use super::{max_path_sum, TreeNode};
    
}
