use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
struct TwoHeapsMidean {
    // left heap will contains items to the left from the median less or equal and keep track of max value in left side
    left: BinaryHeap<i32>,
    // right heap will contains items to the right from the median (greater) and keep track of min value in right side
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
        // we will remove items from heaps only if current peek is in removed map, so complexity of it will be O(1)
        // we need remove map, cause current peek not allways be a needed item to remove, so eventualy peek can become needed item to remove
        // somewhere in the future
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
        // left heap should contain 1 more elements then right, so we pop min from right and push it to the left
        if self.right.len() > self.left.len() {
            self.left.push(self.right.pop().unwrap().0);
        }
    }
    fn push(&mut self, num: i32) {
        self.left.push(num);
        // right will keep track of min peek of max items to the right of the median, so we pop max from left and balance it to the right
        self.right.push(Reverse(self.left.pop().unwrap()));

        self.balance();
    }
    fn balance_madian_push(&mut self, prev: i32, next: i32, prev_median: i32) {
        let mut balance = 0;
        // we will remove prev element from left heap cause left heap contains all numbers to the left of the median inclusevly
        if prev <= prev_median{
            // left heap will lack one item
            balance-=1;
        }else{
            // if prev number is greater then median, then prev number will be remove from right heap, and we need to balance
            balance+=1;
        }
        if next > prev_median{
            // item to push is greater then prev median, we need to push it to the right heap
            self.right.push(Reverse(next));
            balance-=1;
        }else{
            self.left.push(next);
            balance+=1;
        }
        // if balance is ge then 0 then left heap will contains more elements and we need to balance
        if balance>0{
            self.right.push(Reverse(self.left.pop().unwrap()));
        }else if balance<0{
            // else if balance is less then 0 then left heap will lack of number and we need to balance it
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
