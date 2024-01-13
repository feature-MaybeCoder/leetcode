

----

Tags: #leetcode #heap #array #medium

----

## Drawing:
[[find-k-pairs-with-smallest-sums.excalidraw]]

----


## solution explanation:
in worst case we have n decision to make, so we need some priority queue

## last submission:
```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
impl Solution {
   pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    if nums1.is_empty() || nums2.is_empty() || k == 0 {
        return res;
    }
    let mut queue = BinaryHeap::new();

    for j in 0..nums2.len() {
        queue.push(Reverse((nums1[0] + nums2[j], 0, j)));
    }

    for _ in 0..k as usize {
        if let Some(Reverse((_, i, j))) = queue.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if i + 1 < nums1.len() {
                queue.push(Reverse((nums1[i + 1] + nums2[j], i + 1, j)));
            }
        } else {
            break;
        }
    }

    res
}


}
```



