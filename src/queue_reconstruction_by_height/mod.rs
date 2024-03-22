struct BIT_rec {
    tree: Vec<i32>,
    len: usize,
}
impl BIT_rec {
    fn new(len: usize) -> Self {
        let mut bit_len = 1;
        while bit_len < len {
            bit_len *= 2;
        }
        bit_len *= 2;
        let mut tree = vec![0; bit_len];
        Self::construct(0, len - 1, 0, &mut tree);
        Self { tree, len }
    }
    fn construct(left: usize, right: usize, idx: usize, tree: &mut Vec<i32>) -> i32 {
        if left == right {
            tree[idx] = 1;
            return 1;
        }
        let mid = left + right >> 1;
        let mut sum = Self::construct(left, mid, 2 * idx + 1, tree);
        sum += Self::construct(mid + 1, right, 2 * idx + 2, tree);
        tree[idx] = sum;
        return sum;
    }
    fn find(
        left: usize,
        right: usize,
        idx: usize,
        value: i32,
        tree: &mut Vec<i32>,
        index: &mut usize,
    ) {
        if left > right {
            return;
        }
        tree[idx] -= 1;
        if left == right {
            *index = left;
            return;
        }

        let mid = left + right >> 1;
        let l_idx = idx * 2 + 1;
        let diff = tree[l_idx] - value;
        if diff > 0 {
            Self::find(left, mid, l_idx, value, tree, index);
        } else {
            Self::find(
                mid + 1,
                right,
                idx * 2 + 2,
                value - tree[l_idx],
                tree,
                index,
            );
        }
    }
}
struct BIT {
    tree: Vec<i32>,
}
impl BIT {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }
    fn find(&self, mut idx: i32) -> i32 {
        let mut s = 0;
        while idx > 0 {
            
            s += self.tree[idx as usize];
            idx -= idx & -idx;
        }
        
        return s;
    }
    fn update(&mut self, mut idx: usize, val: i32) {
        idx += 1;
        while idx < self.tree.len() {
            self.tree[idx] += val;
            idx += (idx as i32 & -(idx as i32)) as usize;
        }
    }
}
pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![0, 0]; people.len()];
    people.sort_by_key(|item| (item[0], -item[1]));
    let len = res.len() - 1;
    let mut bit = BIT::new(res.len());
    let bs = |cnt: i32, bit_: &BIT| {
        let mut left: i32 = 0;
        let mut right = len as i32;
        while left <= right {
            let mid = left + right >> 1;
            let indexed_mid = mid+1;
            let cnt_query = indexed_mid - bit_.find(indexed_mid);
            if cnt_query >= cnt {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return left;
    };
    for cur in people {
        let idx = bs(cur[1] + 1, &bit) as usize;
        res[idx] = cur;

        bit.update(idx, 1);
    }
    return res;
}

pub fn reconstruct_queue_rec(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![0, 0]; people.len()];
    people.sort_by_key(|item| (item[0], -item[1]));
    let len = res.len() - 1;
    let mut bit = BIT::new(res.len());
    for cur in people {
        let mut index = 0;
        BIT::find(0, len, 0, cur[1], &mut bit.tree, &mut index);
        res[index] = cur
    }
    return res;
}

#[cfg(test)]
mod test {
    use crate::queue_reconstruction_by_height::reconstruct_queue;

    #[test]
    fn base_case() {
        assert_eq!(
            reconstruct_queue(
                [[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]
                    .iter()
                    .map(|item| item.to_vec())
                    .collect()
            ),
            [[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]],
        );
    }
    #[test]
    fn single_item_case() {
        assert_eq!(
            reconstruct_queue([[1, 0]].iter().map(|item| item.to_vec()).collect()),
            [[1, 0]],
        );
    }
}
