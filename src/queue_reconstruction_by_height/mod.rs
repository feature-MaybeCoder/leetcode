pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = Vec::with_capacity(people.len());
    people.sort_by_key(|item| (-item[0], item[1]));
    for cur in people{
        res.insert(cur[1] as usize, cur);
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
}
