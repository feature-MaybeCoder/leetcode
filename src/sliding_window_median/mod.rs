use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
struct TwoHeapsMidean {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
    removed: HashMap<i32, i32>,
    is_odd: bool,
}
impl TwoHeapsMidean {
    fn new(nums: &[i32], len: usize) -> Self {
        let mut two_heaps = Self {
            left: BinaryHeap::with_capacity(nums.len()),
            right: BinaryHeap::with_capacity(nums.len()),
            is_odd: nums.len() % 2 != 0,
            removed: HashMap::with_capacity(len),
        };
        for num in nums {
            two_heaps.push(*num);
        }

        return two_heaps;
    }
    fn median(&self) -> f64 {
        if self.is_odd {
            return *self.left.peek().unwrap() as f64;
        }
        let left = *self.left.peek().unwrap();
        let right = (*self.right.peek().unwrap()).0;

        (left as f64 + right as f64) / 2_f64
    }
    fn remove(&mut self, num: i32) {
        *self.removed.entry(num).or_insert(0) += 1;
        while !self.left.is_empty() && self.removed.contains_key(self.left.peek().unwrap()) {
            let key = self.left.pop().unwrap();
            let num = self.removed.get_mut(&key).unwrap();
            if *num == 1 {
                self.removed.remove(&key);
                continue;
            }
            *num -= 1;
        }
        while !self.right.is_empty() && self.removed.contains_key(&self.right.peek().unwrap().0) {
            let key = self.right.pop().unwrap().0;
            let num = self.removed.get_mut(&key).unwrap();
            if *num == 1 {
                self.removed.remove(&key);
                continue;
            }
            *num -= 1;
        }
    }
    fn balance(&mut self) {
        if self.right.len() > self.left.len() {
            self.left.push(self.right.pop().unwrap().0);
        }
    }
    fn push(&mut self, num: i32) {
        self.left.push(num);
        self.right.push(Reverse(self.left.pop().unwrap()));

        self.balance();
    }
    fn balance_madian_push(&mut self, prev: i32, next: i32, prev_median: i32) {
        let mut balance = 0;
        if prev > prev_median{
            balance +=1;
        }else{
            balance-=1;
        }
        if next > prev_median{
            balance -=1;
            self.right.push(Reverse(next));
            
        }else{
            balance +=1;
            self.left.push(next);
        }
        if balance > 0{
            self.right.push(Reverse(self.left.pop().unwrap()));
            
        }else if balance<0 {
            self.left.push(self.right.pop().unwrap().0);
        }
    }
}

pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let idx_diff = k - 1;
    let idx_bount = nums.len() - 1;
    let mut res = Vec::with_capacity(nums.len());
    let mut heap = TwoHeapsMidean::new(&nums[0..k as usize], nums.len());
    for index in idx_diff..nums.len() {
        let median = heap.median();
        res.push(median);
        if index == idx_bount {
            break;
        }
        let prev = nums[index - idx_diff];
        let next = nums[index + 1];
        heap.balance_madian_push(prev, next, median as i32);
        heap.remove(prev);
    }
    return res;
}
#[cfg(test)]
mod test {
    use super::median_sliding_window;

    #[test]
    fn base_case() {
        let sample = [1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(
            median_sliding_window(sample.to_vec(), 3),
            [1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000]
        );
    }
    #[test]
    fn even_case() {
        let sample = [1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(
            median_sliding_window(sample.to_vec(), 4),
            [0.00000, 1.00000, 1.00000, 4.00000, 5.50000]
        );
    }
    #[test]
    fn odd_positive_case() {
        let sample = [1, 2, 3, 4, 2, 3, 1, 4, 2];
        assert_eq!(
            median_sliding_window(sample.to_vec(), 3),
            [2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000]
        );
    }
    #[test]
    fn int_overflow_case() {
        let sample = [2147483647, 2147483647];
        assert_eq!(
            median_sliding_window(sample.to_vec(), 2),
            [2147483647.00000]
        );
    }
    #[test]
    fn int_negative_overflow_case() {
        let sample = [
            -2147483648,
            -2147483648,
            2147483647,
            -2147483648,
            -2147483648,
            -2147483648,
            2147483647,
            2147483647,
            2147483647,
            2147483647,
            -2147483648,
            2147483647,
            -2147483648,
        ];
        assert_eq!(
            median_sliding_window(sample.to_vec(), 3),
            [
                -2147483648.00000,
                -2147483648.00000,
                -2147483648.00000,
                -2147483648.00000,
                -2147483648.00000,
                2147483647.00000,
                2147483647.00000,
                2147483647.00000,
                2147483647.00000,
                2147483647.00000,
                -2147483648.00000
            ]
        );
    }
}
