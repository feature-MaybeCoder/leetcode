use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Debug)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
#[derive(PartialEq, Debug)]
struct StackNode(MinNonNan, usize);

impl Eq for StackNode {}

impl PartialOrd for StackNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for StackNode {
    fn cmp(&self, other: &StackNode) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start_node: i32,
    end_node: i32,
) -> f64 {
    let mut graph: Vec<Vec<(usize, f64)>> = Vec::with_capacity(n as usize);
    let mut visited: Vec<bool> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        visited.push(false);
        graph.push(Vec::new());
    }
    let stack_cap = edges.len();
    let mut probs = vec![0.0f64; n as usize];
    for (index, edge) in edges.into_iter().enumerate() {
        graph[edge[0] as usize].push((edge[1] as usize, succ_prob[index]));
        graph[edge[1] as usize].push((edge[0] as usize, succ_prob[index]));
    }
    let mut stack: BinaryHeap<StackNode> = BinaryHeap::with_capacity(stack_cap);
    stack.push(StackNode(MinNonNan(1.0), start_node as usize));
    while !stack.is_empty() {
        let stack_node = stack.pop().unwrap();
        let prob = stack_node.0;
        let node = stack_node.1;
        if visited[node] {
            continue;
        }
        visited[node] = true;
        probs[node] = prob.0;
        if node == end_node as usize {
            return prob.0;
        }
        for (neighb, n_prob) in &graph[node] {
            let neighb = *neighb;
            let n_prob = *n_prob;
            if probs[neighb] > n_prob{
                continue
            }
            stack.push(StackNode(MinNonNan(prob.0 * n_prob), neighb));
        }
    }
    return 0.0;
}

#[cfg(test)]
mod test {
    use super::max_probability;

    #[test]
    fn base_case() {
        assert_eq!(
            max_probability(
                3,
                [[0, 1], [1, 2], [0, 2]]
                    .iter()
                    .map(|edge| edge.to_vec())
                    .collect(),
                [0.5, 0.5, 0.2].to_vec(),
                0,
                2,
            ),
            0.25
        );
    }
}
