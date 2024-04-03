use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

use crate::test_helpers::TreeNode;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
type Node = Option<Rc<RefCell<TreeNode>>>;
fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, lca: &mut Option<Rc<RefCell<TreeNode>>>, lca_depth: &mut i32, depth: i32)->i32{
    if node.is_none(){
        return depth;
    }
    let node = node.unwrap();
    
    let right = dfs(node.as_ref().borrow_mut().right.as_ref(), lca, lca_depth,depth+1);
    let left = dfs(node.as_ref().borrow_mut().left.as_ref(), lca, lca_depth,depth+1);
    if right == left && left >= *lca_depth{
        *lca_depth = right;
        *lca = Some(Rc::clone(node));
    }
    return left.max(right);
}
pub fn lca_deepest_leaves(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut lca_depth = -1;
    let mut lca: Option<Rc<RefCell<TreeNode>>> = None;
    dfs(root.as_ref(), &mut lca, &mut lca_depth,0);
    return Some(lca.take().unwrap())
}
#[cfg(test)]
mod test{
    
    use crate::test_helpers::construct_bt_from_traversal;
    use super::{lca_deepest_leaves, TreeNode};
    #[test]
    fn base_case(){
        let tree = construct_bt_from_traversal(0, &vec![Some(2), Some(1), None, None, Some(3), None, None]).1;
        assert_eq!(lca_deepest_leaves(tree.clone()), tree.clone());
    }
}
