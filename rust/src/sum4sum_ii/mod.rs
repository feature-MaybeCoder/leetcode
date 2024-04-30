use std::collections::{HashMap};

pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut last_look_up: HashMap<i32, i32> = HashMap::new();
    
    let mut amount = 0;
    for index in 0..nums3.len() {
        for index_s in 0..nums4.len() {
            let sum = nums3[index] + nums4[index_s];
            if last_look_up.contains_key(&sum){
                last_look_up.insert(sum, last_look_up[&sum]+1);
                continue
            }
            last_look_up.insert(sum, 1);
        }
    }
   
    for index in 0..nums1.len() {
        for index2 in 0..nums2.len() {
            let sum = nums1[index] + nums2[index2];
            let target = 0 - sum;
            if last_look_up.contains_key(&target){
                amount+=last_look_up.get(&target).unwrap();
            }
        }
    }
    
    return amount;
}

#[cfg(test)]
mod test {
    use crate::sum4sum_ii::four_sum_count;

    #[test]
    fn base_case() {
        assert_eq!(
            four_sum_count(
                [1, 2].to_vec(),
                [-2, -1].to_vec(),
                [-1, 2].to_vec(),
                [0, 2].to_vec()
            ),
            2
        );
    }
    #[test]
    fn minus__case() {
        assert_eq!(
            four_sum_count(
                [-1,-1].to_vec(),
                [-1,1].to_vec(),
                [-1,1].to_vec(),
                [1,-1].to_vec()
            ),
            6
        );
    }
}
