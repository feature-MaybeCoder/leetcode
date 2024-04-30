use std::collections::BinaryHeap;

struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::with_capacity(100),
            right_heap: BinaryHeap::with_capacity(100),
        }
    }

    fn add_num(&mut self, num: i32) {
        let left_len = self.left_heap.len();
        let right_len = self.right_heap.len();
        
        if left_len == 0 {
            self.left_heap.push(num);
            return;
        }
        
        let left_peek = *self.left_heap.peek().unwrap();
        
        
        if num > left_peek {
            self.right_heap.push(-num);
            if self.right_heap.len() > left_len + 1 {
                self.left_heap.push(-self.right_heap.pop().unwrap());
            }
            return;
        }
    
        self.left_heap.push(num);
        if left_len + 1 > right_len + 1 {
            self.right_heap.push(-self.left_heap.pop().unwrap());
        }
    }

    fn find_median(&mut self) -> f64 {
        let left_len = self.left_heap.len();
        let right_len = self.right_heap.len();
        let left_val = *self.left_heap.peek().unwrap();

        if left_len > right_len {
            return left_val as f64;
        }
        let right_val = *self.right_heap.peek().unwrap() * -1;
        if right_len > left_len {
            return right_val as f64;
        }
        (left_val as f64 + right_val as f64) / 2.0
    }
}
#[cfg(test)]
mod test {
    use super::MedianFinder;

    #[test]
    fn base_case() {
        let mut finder = MedianFinder::new();
        finder.add_num(3);
        finder.add_num(2);
        finder.add_num(1);
        assert_eq!(finder.find_median(), 2.0);
    }
    #[test]
    fn base_case_reorder() {
        let mut finder = MedianFinder::new();
        finder.add_num(1);
        finder.add_num(2);
        finder.add_num(3);
        assert_eq!(finder.find_median(), 2.0);
    }
    #[test]
    fn even_median() {
        let mut finder = MedianFinder::new();
        finder.add_num(1);
        finder.add_num(2);
        finder.add_num(3);
        finder.add_num(4);
        assert_eq!(finder.find_median(), 2.5);
    }
    #[test]
    fn even_median_reorder() {
        let mut finder = MedianFinder::new();
        finder.add_num(4);
        finder.add_num(3);
        finder.add_num(2);
        finder.add_num(1);
        assert_eq!(finder.find_median(), 2.5);
    }
    #[test]
    fn move_from_left_to_right() {
        let mut finder = MedianFinder::new();
        finder.add_num(12);
        finder.add_num(10);
        finder.add_num(13);
        finder.add_num(11);
        finder.add_num(5);
        assert_eq!(finder.find_median(), 11.0);
    }
    // #[test]
    // fn even_median_reorder_() {
    //     let mut finder = MedianFinder::new();
    //     finder.add_num(40);
    //     finder.add_num(12);
    //     finder.add_num(16);
    //     finder.add_num(14);
    //     finder.add_num(35);
    //     assert_eq!(finder.find_median(), 11.0);
    // }
}
