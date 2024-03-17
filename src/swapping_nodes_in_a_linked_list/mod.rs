use crate::test_helpers::ListNode;

pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut nodes: Vec<Option<Box<ListNode>>> = Vec::with_capacity(100);
    let k = k as usize;
    while let Some(mut node) = head {
        head = node.next.take();
        nodes.push(Some(node));
    }
    let start_i = k - 1;
    let end_i = nodes.len() - k;
    let temp = nodes[start_i].as_ref().unwrap().val;
    nodes[start_i].as_mut().unwrap().val = nodes[end_i].as_ref().unwrap().val;
    nodes[end_i].as_mut().unwrap().val = temp;
    
    
    let c_head = nodes[0].as_mut();
    
    for index in 0..nodes.len(){
        c_head.unwrap().next = Some(nodes[index].unwrap());
    }
    return nodes[0].take();
}

#[cfg(test)]
mod test{
    use crate::test_helpers::construct_list;

    use super::swap_nodes;

    #[test]
    fn base_case(){
        let list = construct_list(vec![1,2,3,4]);
        swap_nodes(list, 2);
    }
}
