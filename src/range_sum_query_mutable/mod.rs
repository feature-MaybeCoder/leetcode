struct NumArrayPrefix {
    nums: Vec<i32>,
    prefix: Vec<i32>,
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
impl NumArrayPrefix {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(nums.len());
        let mut sum = 0;
        for num in &nums {
            sum += num;
            prefix.push(sum);
        }
        return Self { nums, prefix };
    }

    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let diff = val - self.nums[index];
        self.nums[index] = val;
        for idx in index..self.nums.len() {
            self.prefix[idx] += diff;
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let mut sum = self.prefix[right];
        if left > 0 {
            sum -= self.prefix[left - 1];
        }
        return sum;
    }
}
struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>,
    len: usize,
}
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = vec![0; nums.len() * 4];
        Self::construct(&mut tree, 0, &nums, 0, nums.len() - 1);
        return Self {
            tree,
            len: nums.len() - 1,
            nums,
        };
    }
    fn construct(
        tree: &mut Vec<i32>,
        tree_idx: usize,
        arr: &Vec<i32>,
        left: usize,
        right: usize,
    ) -> i32 {
        if left == right {
            tree[tree_idx] = arr[left];
            return tree[tree_idx];
        }
        let mut sum = 0;
        let mid = (left + right) >> 1;
        sum += Self::construct(tree, 2 * tree_idx + 1, arr, left, mid);
        sum += Self::construct(tree, 2 * tree_idx + 2, arr, mid + 1, right);
        tree[tree_idx] = sum;
        return sum;
    }
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let diff = val - self.nums[index as usize];
        self.nums[index] = val;
        self.update_tree(0, 0, self.len, index, diff);
    }
    fn update_tree(
        &mut self,
        tree_index: usize,
        left: usize,
        right: usize,
        index: usize,
        diff: i32,
    ) {
        if index < left || index > right {
            return;
        }
        if left == right{
            self.tree[tree_index] += diff;
            return
        }
        self.tree[tree_index] += diff;
        let mid = left + right >> 1;
        self.update_tree(2 * tree_index + 1, left, mid, index, diff);
        self.update_tree(2 * tree_index + 2, mid + 1, right, index, diff);
    }
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        Self::find(&self, 0, 0, self.len, left, right)
    }
    fn find(
        &self,
        tree_index: usize,
        tree_left: usize,
        tree_right: usize,
        left: usize,
        right: usize,
    ) -> i32 {
        if tree_right < left || tree_left > right {
            return 0;
        }
        if tree_left >= left && tree_right <= right {
            return self.tree[tree_index];
        }
        let mut sum = 0;
        let mid = tree_left + tree_right >> 1;
        sum += Self::find(&self, 2 * tree_index + 1, tree_left, mid, left, right);
        sum += Self::find(&self, 2 * tree_index + 2, mid + 1, tree_right, left, right);
        return sum;
    }
}
#[cfg(test)]
mod test {
    use super::NumArray;

    #[test]
    fn base_case() {
        let tree = NumArray::new(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(tree.sum_range(1, 4), 14);
    }
    #[test]
    fn segment_tree_construction() {
        let tree = NumArray::new(vec![1, 2, 3]);
        assert_eq!(tree.sum_range(1, 2), 5);
    }
    #[test]
    fn update_segment_tree() {
        let mut tree = NumArray::new(vec![7,2,7,2,0]);
        
        println!("{:?}",tree.tree);
        // tree.update(4, 6);
        tree.update(0, 2);
        tree.update(0, 8);
        
        println!("{:?}",tree.tree);
        assert_eq!(tree.sum_range(4, 4), 0);
        tree.update(3, 8);
        println!("{:?}",tree.tree);
        assert_eq!(tree.sum_range(0, 4), 25);
    }
}
