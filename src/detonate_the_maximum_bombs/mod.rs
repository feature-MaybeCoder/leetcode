fn merge_bomb(
    left: &Vec<i32>,
    right: &Vec<i32>,
    left_i: usize,
    right_i: usize,
    graph: &mut Vec<Vec<usize>>,
) {
    let x_i = left[0] as i64; // squared value may not fit in i32
    let y_i = left[1] as i64;
    let r_i = (left[2] as i64) * (left[2] as i64);

    let x_j = right[0] as i64;
    let y_j = right[1] as i64;
    let r_j = right[2] as i64;

    let d = (x_i - x_j) * (x_i - x_j) + (y_i - y_j) * (y_i - y_j);
    if d <= r_i {
        graph[left_i].push(right_i);
    }
    if d <= r_j * r_j {
        graph[right_i].push(left_i);
    }
}
fn dfs(node: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) -> i32 {
    if visited[node] {
        return 0;
    }
    visited[node] = true;
    let mut depth = 1;

    for neib in &graph[node] {
        depth += dfs(*neib, visited, graph);
    }

    return depth;
}
pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let len = bombs.len();
    let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(len - 1); len];
    let mut visited: Vec<bool> = vec![false; len];
    let mut max = 1;
    for left in 0..len {
        let left_bom = &bombs[left];
        for right in left + 1..len {
            let right_bomb = &bombs[right];
            merge_bomb(left_bom, right_bomb, left, right, &mut graph);
        }
    }
    for node in 0..len {
        let depth = dfs(node, &mut visited, &graph);
        if depth > max {
            max = depth;
        }
        visited.fill(false);
    }
    return max;
}

#[cfg(test)]
mod test {
    use super::maximum_detonation;

    #[test]
    fn base_case() {
        assert_eq!(
            maximum_detonation(
                [[2, 1, 3], [6, 1, 4]]
                    .iter()
                    .map(|edge| edge.to_vec())
                    .collect()
            ),
            2
        );
    }
    #[test]
    fn multi_bomb() {
        assert_eq!(
            maximum_detonation(
                [[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]]
                    .iter()
                    .map(|edge| edge.to_vec())
                    .collect()
            ),
            5
        );
    }
    #[test]
    fn edge_radius() {
        assert_eq!(
            maximum_detonation(
                [[1, 1, 100000], [100000, 100000, 1]]
                    .iter()
                    .map(|edge| edge.to_vec())
                    .collect()
            ),
            1
        );
    }
    #[test]
    fn merge_case() {
        assert_eq!(
            maximum_detonation(
                [
                    [56, 80, 2],
                    [55, 9, 10],
                    [32, 75, 2],
                    [87, 89, 1],
                    [61, 94, 3],
                    [43, 82, 9],
                    [17, 100, 6],
                    [50, 6, 7],
                    [9, 66, 7],
                    [98, 3, 6],
                    [67, 50, 2],
                    [79, 39, 5],
                    [92, 60, 10],
                    [49, 9, 9],
                    [42, 32, 10]
                ]
                .iter()
                .map(|edge| edge.to_vec())
                .collect()
            ),
            3
        );
    }
    #[test]
    fn merge_case_2() {
        assert_eq!(
            maximum_detonation(
                [
                    [63, 47, 3],
                    [94, 76, 1],
                    [38, 53, 5],
                    [66, 67, 9],
                    [35, 64, 10],
                    [43, 46, 1],
                    [76, 95, 9],
                    [62, 94, 3],
                    [42, 67, 7],
                    [19, 84, 7],
                    [80, 16, 9],
                    [7, 81, 4],
                    [67, 25, 5],
                    [32, 27, 1],
                    [2, 32, 10],
                    [17, 46, 6],
                    [40, 32, 6]
                ]
                .iter()
                .map(|edge| edge.to_vec())
                .collect()
            ),
            2
        );
    }
    #[test]
    fn merge_case_3() {
        assert_eq!(
            maximum_detonation(
                [
                    [855, 82, 158],
                    [17, 719, 430],
                    [90, 756, 164],
                    [376, 17, 340],
                    [691, 636, 152],
                    [565, 776, 5],
                    [464, 154, 271],
                    [53, 361, 162],
                    [278, 609, 82],
                    [202, 927, 219],
                    [542, 865, 377],
                    [330, 402, 270],
                    [720, 199, 10],
                    [986, 697, 443],
                    [471, 296, 69],
                    [393, 81, 404],
                    [127, 405, 177]
                ]
                .iter()
                .map(|edge| edge.to_vec())
                .collect()
            ),
            9
        );
    }
}
