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
    fn inorder(&self, node: &Option<Rc<RefCell<TreeNode>>>, serialize_str: &mut String) {
        if let Some(node) = node {
            let cur = node.borrow();
            self.inorder(&cur.left, serialize_str);
            let mut val_str = cur.val.to_string();
            val_str.push(',');
            serialize_str.push_str(&val_str);
            self.inorder(&cur.right, serialize_str);
        }
    }
    fn postorder(&self, node: &Option<Rc<RefCell<TreeNode>>>, serialize_str: &mut String) {
        if let Some(node) = node {
            let cur = node.borrow();
            let mut val_str = cur.val.to_string();
            val_str.push(',');
            self.postorder(&cur.left, serialize_str);
            self.postorder(&cur.right, serialize_str);
            serialize_str.push_str(&val_str);
        }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        self.inorder(&root, &mut res);
        if res.len() > 0 {
            res.pop();
        }
        res.push(';');
        self.postorder(&root, &mut res);
        if res.len() > 1 {
            res.pop();
        }
        return res;
    }
    fn construct_bst_from_traversals(
        &self,
        i_start: i32,
        i_end: i32,
        inorder: &Vec<i32>,
        postorder: &mut Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if (i_end - i_start) < 0 {
            return None;
        }
        println!("{:?}, {:?}, {:?}, {:?}", inorder,postorder, i_start, i_end);
        let head_val = postorder.pop().unwrap();
        let inorder_index = inorder.iter().position(|val| *val == head_val).unwrap() as i32;
        let mut root = TreeNode::new(head_val);
        root.right =
            self.construct_bst_from_traversals(inorder_index + 1, i_end, inorder, postorder);
        root.left =
            self.construct_bst_from_traversals(i_start, inorder_index - 1, inorder, postorder);
        return Some(Rc::new(RefCell::new(root)));
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let splited: Vec<&str> = data.split(';').collect();
        
        if splited[0].is_empty(){
            return None
        }
        let inorder: Vec<_> = splited[0]
            .split(',')
            .map(|char| char.parse::<i32>().unwrap())
            .collect();
        let mut postorder: Vec<_> = splited[1]
            .split(',')
            .map(|char| char.parse::<i32>().unwrap())
            .collect();
        return self.construct_bst_from_traversals(0, inorder.len() as i32 - 1, &inorder, &mut postorder);
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

        assert_eq!(serialized, "1,2,3;1,3,2");
        serializer.deserialize(serialized);
    }
    #[test]
    fn none_edge_case() {
        let serializer = Codec::new();
        
        let serialized = serializer.serialize(None);
        assert_eq!(serialized, ";");
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
        assert_eq!(serialized, "2,1,2;2,2,1");
        serializer.deserialize(serialized);
    }
}
