use crate::test_helpers::ListNode;
fn add_arrs(arr_1: &Vec<i32>, arr_2: &Vec<i32>) -> Option<Box<ListNode>> {
    if arr_2.len() > arr_1.len() {
        return add_arrs(arr_2, arr_1);
    }
    let mut remain = 0;
    let mut index_1 = arr_1.len() - 1;
    let mut index_2 = arr_2.len() - 1;
    let mut head: Option<Box<ListNode>> = None;

    loop {
        let mut sum = arr_1[index_1] + arr_2[index_2] + remain;
        remain = 0;
        if sum >= 10 {
            sum -= 10;
            remain = 1;
        }
        head = Some(Box::new(ListNode {
            next: head,
            val: sum,
        }));
        if index_2 == 0 {
            break;
        }
        index_1 -= 1;
        index_2 -= 1;
    }

    while index_1 > 0 {
        index_1 -= 1;

        let mut sum = arr_1[index_1] + remain;
        remain = 0;
        if sum >= 10 {
            sum -= 10;
            remain = 1
        }
        head = Some(Box::new(ListNode {
            next: head,
            val: sum,
        }));
    }
    if remain > 0 {
        head = Some(Box::new(ListNode { next: head, val: 1 }));
    }
    return head;
}
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1_vals: Vec<i32> = Vec::with_capacity(100);
    let mut l2_vals: Vec<i32> = Vec::with_capacity(100);
    while l1.is_some() || l2.is_some() {
        if l1.is_some() {
            let node = l1.take().unwrap();
            l1_vals.push(node.val);
            l1 = node.next;
        }
        if l2.is_some() {
            let node = l2.take().unwrap();
            l2_vals.push(node.val);
            l2 = node.next;
        }
    }
    add_arrs(&l1_vals, &l2_vals)
}
#[cfg(test)]
mod test {
    use crate::test_helpers::construct_list;

    use super::add_two_numbers;

    #[test]
    fn base_case() {
        let l1 = construct_list(vec![1, 2, 3]);
        let l2 = construct_list(vec![1, 2, 3]);
        let ret_list = construct_list(vec![2, 4, 6]);
        assert_eq!(add_two_numbers(l1, l2), ret_list);
    }
    #[test]
    fn remain_case() {
        let l1 = construct_list(vec![8, 9, 9]);
        let l2 = construct_list(vec![2]);
        let ret_list = construct_list(vec![9, 0, 1]);
        assert_eq!(add_two_numbers(l1, l2), ret_list);
    }
}
