use std::collections::HashMap;

type Cache = HashMap<i32, Vec<Vec<i32>>>;

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let power: usize = 2;
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(power.pow(nums.len() as u32));
    let mut cache: Cache = HashMap::with_capacity(nums.len());
    for num in &nums {
        if cache.contains_key(num) {
            let vec = cache.get_mut(num).unwrap();
            for vec_n in vec {
                vec_n.push(*num);
            }
            cache.get_mut(num).unwrap().push(vec![*num]);
        } else {
            cache.insert(*num, vec![vec![*num]]);
        }
    }

    for (index, mut vec) in cache.into_iter() {
        for res_idx in 0..res.len() {
            for num_vec in &vec {
                let mut v = res[res_idx].clone();
                let mut v_2 = num_vec.clone();
                v_2.append(&mut v);
                res.push(v_2);
            }
        }

        res.append(&mut vec);
    }
    res.push(vec![]);

    return res;
}

#[cfg(test)]
mod test {
    use super::subsets_with_dup;

    #[test]
    fn base_case() {
        let assert_val: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];
        assert_eq!(subsets_with_dup(vec![1, 2, 2]), assert_val);
    }
}
