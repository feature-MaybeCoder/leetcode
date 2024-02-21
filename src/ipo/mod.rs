use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Debug)]
struct StackNode(i32, usize);
impl Eq for StackNode {}

impl PartialOrd for StackNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for StackNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
fn pop_untill_limit(queue: &mut BinaryHeap<StackNode>, profit_q: &mut BinaryHeap<i32>, w: i32, profit: &Vec<i32>)->Option<StackNode>{
    
    while let Some(node) = queue.pop() {
        let cap = node.0 * -1;
        if cap > w{
            return Some(node)
        }
        profit_q.push(profit[node.1]);
    }
    
    return None
}
pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let len = profits.len();
    let mut profits_q: BinaryHeap<i32> = BinaryHeap::with_capacity(len);
    let mut capital_q: BinaryHeap<StackNode> = BinaryHeap::with_capacity(len);
    for (index, value) in capital.iter().enumerate(){
        capital_q.push(StackNode(-value, index));
    }
    let mut next_limit = pop_untill_limit(&mut capital_q, &mut profits_q, w, &profits);
    while k >0 && !profits_q.is_empty() {
        w += profits_q.pop().unwrap();
        k-=1;
        if next_limit.is_some() {
            let next = next_limit.as_ref().unwrap();
            if -next.0 > w {
                continue;
            }else{
                profits_q.push(profits[next.1]);
                next_limit =  pop_untill_limit(&mut capital_q, &mut profits_q, w, &profits);
            }
            
        }
    }
    
    
    return w
}

#[cfg(test)]
mod test{
    use super::find_maximized_capital;

    #[test]
    fn base_case(){
        assert_eq!(find_maximized_capital(2, 0, vec![1,2,3], vec![0,1,1]), 4);
    }
}
