use std::cell::Cell;

fn tarjans_dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    disc: &mut Vec<i32>,
    low: &mut Vec<i32>,
    parent: usize,
    critical: &mut Vec<Vec<i32>>,
    disc_counter: &mut i32,
) {
    disc[node] = *disc_counter;
    low[node] = *disc_counter;
    *disc_counter += 1;
    for neighboor in &graph[node] {
        let node_neighboor = *neighboor;
        if parent == node_neighboor {
            continue;
        }
        if disc[node_neighboor] != -1 {
            low[node] = low[node].min(low[node_neighboor]);
            continue;
        }

        tarjans_dfs(
            node_neighboor,
            graph,
            disc,
            low,
            node,
            critical,
            disc_counter,
        );
        low[node] = low[node].min(low[node_neighboor]);
        if low[node_neighboor] > disc[node] {
            critical.push(vec![node as i32, node_neighboor as i32]);
        }
    }
}
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(n as usize);
    let mut disc: Vec<i32> = Vec::with_capacity(n as usize);
    let mut low: Vec<i32> = Vec::with_capacity(n as usize);

    let mut ciritical: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        disc.push(-1);
        low.push(-1);

        graph.push(Vec::new());
    }
    for connection in connections {
        graph[connection[0] as usize].push(connection[1] as usize);
        graph[connection[1] as usize].push(connection[0] as usize);
    }
    let mut counter = 0;
    tarjans_dfs(
        0,
        &graph,
        &mut disc,
        &mut low,
        0,
        &mut ciritical,
        &mut counter,
    );

    return ciritical;
}
#[cfg(test)]
mod test {
    use crate::critical_connections_in_a_network::critical_connections;
    #[test]
    fn basic_graph_one_critical() {
        assert_eq!(
            critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]),
            [[1, 3]]
        )
    }
    #[test]
    fn edge_case_graph_two_critical() {
        assert_eq!(
            critical_connections(
                5,
                vec![
                    vec![1, 0],
                    vec![2, 0],
                    vec![3, 2],
                    vec![4, 2],
                    vec![4, 3],
                    vec![3, 0],
                    vec![4, 0]
                ]
            ),
            [[0, 1]]
        )
    }
    #[test]
    fn non_critical_case() {
        let assert: Vec<Vec<i32>> = vec![];
        assert_eq!(
            critical_connections(
                10,
                vec![
                    vec![1, 0],
                    vec![2, 0],
                    vec![3, 0],
                    vec![4, 1],
                    vec![5, 3],
                    vec![6, 1],
                    vec![7, 2],
                    vec![8, 1],
                    vec![9, 6],
                    vec![9, 3],
                    vec![3, 2],
                    vec![4, 2],
                    vec![7, 4],
                    vec![6, 2],
                    vec![8, 3],
                    vec![4, 0],
                    vec![8, 6],
                    vec![6, 5],
                    vec![6, 3],
                    vec![7, 5],
                    vec![8, 0],
                    vec![8, 5],
                    vec![5, 4],
                    vec![2, 1],
                    vec![9, 5],
                    vec![9, 7],
                    vec![9, 4],
                    vec![4, 3]
                ]
            ),
            assert
        );
    }
}
