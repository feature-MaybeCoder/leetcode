use std::collections::VecDeque;

pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(patience.len());
    let mut visited_min: Vec<i32> = Vec::with_capacity(patience.len());

    for _ in 0..patience.len() {
        visited_min.push(-1);
        graph.push(Vec::new());
    }
    let cap = edges.len();
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        graph[from].push(to);
        graph[to].push(from);
    }

    let mut queue: VecDeque<usize> = VecDeque::with_capacity(cap);
    queue.push_back(0);
    visited_min[0] = 0;
    let mut depth = 0;
    let mut max_delay = 0;
    while !queue.is_empty() {
        let queue_len = queue.len();
        for _ in 0..queue_len {
            let node = queue.pop_front().unwrap();
            let mut delay = depth * 2;
            let p = patience[node];
            if delay == 0 || delay % p == 0 {
                delay -= p;
            } else {
                delay -= delay % p;
            }
            delay += depth * 2;
            if delay > max_delay {
                max_delay = delay;
            }
            for neib in &graph[node] {
                let neib = *neib;
                if visited_min[neib] != -1 {
                    continue;
                }
                visited_min[neib] = depth + 1;
                queue.push_back(neib);
            }
        }
        depth += 1;
    }

    return max_delay + 1;
}

#[cfg(test)]
mod test {
    use super::network_becomes_idle;

    #[test]
    fn base_case() {
        assert_eq!(
            network_becomes_idle(
                [[0, 1], [1, 2]].iter().map(|edge| edge.to_vec()).collect(),
                vec![0, 2, 3]
            ),
            8
        );
    }
    #[test]
    fn loop_case() {
        assert_eq!(
            network_becomes_idle(
                [[0, 1], [0, 2], [1, 2]]
                    .iter()
                    .map(|edge| edge.to_vec())
                    .collect(),
                vec![0, 10, 10]
            ),
            3
        );
    }
    #[test]
    fn delay() {
        assert_eq!(
            network_becomes_idle(
                [
                    [5, 7],
                    [15, 18],
                    [12, 6],
                    [5, 1],
                    [11, 17],
                    [3, 9],
                    [6, 11],
                    [14, 7],
                    [19, 13],
                    [13, 3],
                    [4, 12],
                    [9, 15],
                    [2, 10],
                    [18, 4],
                    [5, 14],
                    [17, 5],
                    [16, 2],
                    [7, 1],
                    [0, 16],
                    [10, 19],
                    [1, 8]
                ]
                .iter()
                .map(|edge| edge.to_vec())
                .collect(),
                vec![0, 2, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1]
            ),
            67
        );
    }
}
