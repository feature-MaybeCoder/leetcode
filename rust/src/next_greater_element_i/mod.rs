use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums2.len());
    let mut stack: Vec<i32> = Vec::with_capacity(nums2.len());
    for num in nums2.into_iter() {
        while !stack.is_empty() && *stack.last().unwrap() < num {
            map.insert(stack.pop().unwrap(), num);
        }
        stack.push(num);
    }
    let mut res = vec![-1; nums1.len()];
    for (idx, num) in nums1.iter().enumerate() {
        res[idx] = *map.get(num).unwrap_or(&-1);
    }

    return res;
}
#[cfg(test)]
mod test {
    use super::next_greater_element;

    #[test]
    fn base_case() {
        assert_eq!(
            next_greater_element(vec![1, 3, 5, 2, 4], vec![6, 5, 4, 3, 2, 1, 7]),
            [7, 7, 7, 7, 7]
        );
    }
}
