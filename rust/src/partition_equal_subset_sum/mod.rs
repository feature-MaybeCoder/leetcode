use std::collections::HashSet;
pub fn can_partition(mut nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum::<i32>();
    if sum %2 !=0{
        return false
    }
    let target = sum/2;
    let mut target_sums: HashSet<i32> = HashSet::new();
    let mut items: Vec<i32> = Vec::new();
    target_sums.insert(0);
    for num in nums{
        for idx in 0..items.len(){
            let new_item = num + items[idx];
            
            if new_item == target{
                return true
            }
            if target_sums.contains(&new_item){
                continue;
            }
            items.push(new_item);
            target_sums.insert(new_item);   
        }
    }
    return target_sums.contains(&target)
}
#[cfg(test)]
mod test {
    use super::can_partition;

    #[test]
    fn base_case() {
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
    }
    #[test]
    fn falsy_case() {
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
    }
    #[test]
    fn single_item_case() {
        assert_eq!(can_partition(vec![1]), false);
    }
    #[test]
    fn even_items_case() {
        assert_eq!(can_partition(vec![1, 1, 1, 1]), true);
    }
    #[test]
    fn falsy_even_items_case() {
        assert_eq!(can_partition(vec![1, 3, 4, 4]), false);
    }
    #[test]
    fn mirror_case() {
        assert_eq!(can_partition(vec![1, 1, 2, 2]), true);
    }
    #[test]
    fn duplicates_case() {
        assert_eq!(can_partition(vec![3, 3, 3, 4, 5]), true);
    }
    #[test]
    fn right_rebalance_case() {
        assert_eq!(can_partition(vec![20,10,9,8,8,3]), true);
    }
    #[test]
    fn edge_case() {
        assert_eq!(can_partition(vec![1,2,3,4,5,6,7]), true);
    }
}
