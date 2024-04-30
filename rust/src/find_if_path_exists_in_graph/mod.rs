use std::collections::VecDeque;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    if source == destination {
        return true;
    }
    let mut graph = vec![Vec::new(); n as usize];
    let mut visited = vec![false; n as usize];
    let len = edges.len();
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        graph[from].push(to);
        graph[to].push(from);
    }
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(len);
    queue.push_back(source as usize);
    while let Some(node) = queue.pop_front() {
        for neighb in &graph[node] {
            let neighb = *neighb;
            if neighb as i32 == destination {
                return true;
            }
            if visited[neighb] {
                continue;
            }
            visited[neighb] = true;
            queue.push_back(neighb);
        }
    }
    return false;
}
#[cfg(test)]
mod test {
    use super::valid_path;

    #[test]
    fn base_case() {
        let edges = [[0, 1], [1, 2], [2, 0]];
        assert_eq!(
            valid_path(
                3,
                edges.into_iter().map(|item| item.to_vec()).collect(),
                0,
                2,
            ),
            true
        );
    }
}
