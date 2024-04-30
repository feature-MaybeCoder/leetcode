use crate::test_helpers::ListNode;
fn dfs(head: &mut &Box<ListNode>, cur: &Option<Box<ListNode>>, max: &mut i32, is_exit: &mut bool) {
    if cur.is_none() {
        return;
    }
    let cur = cur.as_ref().unwrap();
    dfs(head, &cur.next, max, is_exit);
    if *is_exit == true {
        return;
    }
    let mut sum = cur.val + head.val;
    *max = *max.max(&mut sum);
    let next_head = head.next.as_ref().unwrap();
    if next_head == cur {
        *is_exit = true;
        return;
    }
    *head = next_head;
}
pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut max = 0;
    let mut is_exit = false;
    dfs(&mut head.as_ref().unwrap(), &head, &mut max, &mut is_exit);
    max
}
#[cfg(test)]
mod test {
    use crate::test_helpers::construct_list;

    use super::pair_sum;

    #[test]
    fn base_case() {
        assert_eq!(pair_sum(construct_list(vec![1, 2, 10, 10, 2, 3])), 20)
    }
    #[test]
    fn small_case() {
        assert_eq!(pair_sum(construct_list(vec![5, 4, 2, 1])), 6)
    }
}
