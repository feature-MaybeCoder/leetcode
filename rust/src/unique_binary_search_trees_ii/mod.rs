use std::cell::RefCell;
use std::rc::Rc;
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
fn dfs(from: i32, to: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if from > to {
        return vec![None];
    }
    let mut res: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    for index in from..=to {
        
        let r_subtree = dfs(index+1, to);
        let l_subtree = dfs(from, index-1);
       
        for r_subtree_root in &r_subtree {
            for l_subtree_root in &l_subtree {
                let mut root = TreeNode::new(index );
                root.right = r_subtree_root.clone();
                root.left = l_subtree_root.clone();
                res.push(Some(Rc::new(RefCell::new(root))))
            }
        }
    }
    return res;
}
pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n == 0{
        return vec![None]
    }
    return dfs(1, n)
}

#[cfg(test)]
mod test {
    use super::generate_trees;

    #[test]
    fn basic_trees_case() {
        assert_eq!(generate_trees(3).len(), 5);
    }
}
