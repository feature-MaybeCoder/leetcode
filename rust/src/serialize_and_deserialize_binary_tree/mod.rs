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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        return Self {};
    }
    fn dfs(&self, node: &Option<Rc<RefCell<TreeNode>>>, ser_out: &mut String) {
        if let Some(node) = node {
            let unwraped = node.borrow();
            let mut ser_value = unwraped.val.to_string();
            ser_value.push(',');
            ser_out.push_str(&ser_value);
            self.dfs(&unwraped.left, ser_out);
            self.dfs(&unwraped.right, ser_out);
            return;
        }
        ser_out.push_str("null,")
    }
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        self.dfs(&root, &mut res);
        if !res.is_empty() {
            res.pop();
        }
        return res;
    }
    fn construct_bt_from_traversal(
        &self,
        index: usize,
        traversal: &Vec<Option<i32>>,
    ) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(val) = traversal[index] {
            let mut node = TreeNode::new(val);
            let (left_index, left_node) = self.construct_bt_from_traversal(index + 1, traversal);
            let (right_index, right_node) =
                self.construct_bt_from_traversal(left_index + 1, traversal);

            node.left = left_node;
            node.right = right_node;

            return (right_index, Some(Rc::new(RefCell::new(node))));
        }
        return (index, None);
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let splited: Vec<_> = data
            .split(',')
            .map(|item| match item {
                "null" => None,
                _ => Some(item.parse::<i32>().unwrap()),
            })
            .collect();
        return self.construct_bt_from_traversal(0, &splited).1;
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::{Codec, TreeNode};

    #[test]
    fn basic_serialization_test() {
        let serializer = Codec::new();
        let mut root_node = TreeNode::new(2);
        root_node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root_node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(root_node)));
        let serialized = serializer.serialize(root);

        assert_eq!(serialized, "2,1,null,null,3,null,null");
        serializer.deserialize(serialized);
    }
    #[test]
    fn none_edge_case() {
        let serializer = Codec::new();

        let serialized = serializer.serialize(None);
        assert_eq!(serialized, "null");
        serializer.deserialize(serialized);
    }
    #[test]
    fn duplicate_edge_case() {
        let serializer = Codec::new();
        let mut root_node = TreeNode::new(1);
        root_node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root_node.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = Some(Rc::new(RefCell::new(root_node)));
        let serialized = serializer.serialize(root);
        assert_eq!(serialized, "1,2,null,null,2,null,null");
        serializer.deserialize(serialized);
    }
}
