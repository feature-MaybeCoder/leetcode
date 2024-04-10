use std::{cmp::Reverse, collections::BinaryHeap};

type Node = (i32, usize, usize);
type Graph = Vec<Vec<Node>>;
type Visited = Vec<bool>;
type Queue = BinaryHeap<Reverse<Node>>;
fn prims(
    graph: &Graph,
    ignored_idx: usize,
    visited: &mut Visited,
    queue: &mut Queue,
    init_cost: i32,
    idle_cost: i32,
) -> (i32, usize) {
    let mut cost = init_cost;
    let mut visited_am = 1;
    let visited_len = visited.len();
    while !queue.is_empty() && visited_am < visited_len && cost <= idle_cost {
        let (weight, node, _) = queue.pop().unwrap().0;
        if visited[node] {
            continue;
        }
        cost += weight;
        visited_am += 1;
        visited[node] = true;
        for edge in &graph[node] {
            if visited[edge.1] || edge.2 == ignored_idx {
                continue;
            }
            queue.push(Reverse(*edge));
        }
    }
    return (cost, visited_am);
}
struct UnionFind {
    len: i32,
    p: Vec<usize>,
    weights: Vec<i32>,
}
impl UnionFind {
    fn new(len: usize) -> Self {
        let mut p = Vec::with_capacity(len);
        for idx in 0..len {
            p.push(idx);
        }
        return Self {
            len: len as i32 -1,
            p,
            weights: vec![1; len],
        };
    }
    fn find(&mut self, mut node: usize) -> usize {
        while node != self.p[node] {
            self.p[node] = self.p[self.p[node]];
            node = self.p[node];
        }
        return node;
    }
    fn union(&mut self, node1: usize, node2: usize) -> bool {
        let p1 = self.find(node1);
        let p2 = self.find(node2);
        if p1 == p2 {
            return false;
        }
        self.len -= 1;
        if self.weights[p1] > self.weights[p2] {
            self.p[p2] = p1;
            self.weights[p1] += self.weights[p2];
        } else {
            self.p[p1] = p2;
            self.weights[p2] += self.weights[p1];
        }
        return true;
    }
    fn reset(&mut self) {
        self.len = self.weights.len() as i32 -1;
        for idx in 0..self.weights.len() {
            self.weights[idx] = 1;
            self.p[idx] = idx;
        }
    }
    fn find_mst(&mut self, edges: &Vec<Vec<i32>>, edges_len: usize, ignored_idx: i32) -> i32 {
        let mut mst = 0;
        let mut edge_idx: usize = 0;
        while self.len > 0 && edge_idx < edges_len {
            
            let edge = &edges[edge_idx];
            if edge[3] == ignored_idx{
                edge_idx += 1;
                continue;
            }
            if self.union(edge[0] as usize, edge[1] as usize) {
                mst += edge[2];
            }
            edge_idx += 1;
        }
        return mst;
    }
}
pub fn find_critical_and_pseudo_critical_edges(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let edges_len = edges.len();
    for idx in 0..edges_len {
        edges[idx].push(idx as i32);
    }
    let mut critical = Vec::with_capacity(edges_len);
    let mut non_critical = Vec::with_capacity(edges_len);
    let mut uf = UnionFind::new(n);
    edges.sort_by_key(|edge| edge[2]);
    let mst = uf.find_mst(&edges, edges_len, -1);
    for edge_idx in 0..edges_len {
        uf.reset();
        let mut cur_mst = 0;
        let cur_edge = &edges[edge_idx];
        cur_mst += cur_edge[2];
        uf.union(cur_edge[0] as usize,  cur_edge[1] as usize);
        cur_mst += uf.find_mst(&edges, edges_len, -1);
        if cur_mst > mst{
            continue;
        }
        uf.reset();
        let is_critical_mst = uf.find_mst(&edges, edges_len, edges[edge_idx][3]);
        if uf.len > 0 || is_critical_mst>mst{
            critical.push(edges[edge_idx][3]);
            continue
        }
        non_critical.push(edges[edge_idx][3]);
    }
    return vec![critical, non_critical];
}
pub fn find_critical_and_pseudo_critical_edges_prims(
    n: i32,
    edges: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let n = n as usize;
    let edges_len = edges.len();
    let mut graph: Graph = vec![Vec::with_capacity(edges_len); n];
    let mut critical = Vec::with_capacity(edges_len);
    let mut non_critical = Vec::with_capacity(edges_len);
    for idx in 0..edges_len {
        let edge = &edges[idx];
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        let weight = edge[2];
        graph[from].push((weight, to, idx));
        graph[to].push((weight, from, idx));
    }
    let mut visited: Visited = vec![false; n];
    let mut queue: Queue = BinaryHeap::with_capacity(edges_len);
    for edge in &graph[0] {
        queue.push(Reverse(*edge));
    }
    visited[0] = true;
    let (idle_cost, _) = prims(&graph, edges_len, &mut visited, &mut queue, 0, i32::MAX);
    for edge in &graph[0] {
        queue.push(Reverse(*edge));
    }
    for idx in 0..edges_len {
        queue.clear();
        for visit_idx in 0..n {
            visited[visit_idx] = false;
        }
        let from = edges[idx][0] as usize;
        let to = edges[idx][1] as usize;
        visited[from] = true;
        visited[to] = true;
        for edge in &graph[from] {
            queue.push(Reverse(*edge));
        }
        for edge in &graph[to] {
            queue.push(Reverse(*edge));
        }
        let (new_cost_cr, _) = prims(
            &graph,
            edges_len,
            &mut visited,
            &mut queue,
            edges[idx][2],
            idle_cost,
        );
        if new_cost_cr > idle_cost {
            continue;
        }
        for visit_idx in 0..n {
            visited[visit_idx] = false;
        }
        queue.clear();
        for edge in &graph[0] {
            if edge.2 == idx {
                continue;
            }
            queue.push(Reverse(*edge));
        }
        visited[0] = true;
        let (new_cost, visited_am) = prims(&graph, idx, &mut visited, &mut queue, 0, idle_cost);

        if new_cost > idle_cost || visited_am < n {
            critical.push(idx as i32);
        } else {
            non_critical.push(idx as i32);
        }
    }

    return vec![critical, non_critical];
}
#[cfg(test)]
mod test {
    use super::find_critical_and_pseudo_critical_edges;

    #[test]
    fn base_case() {
        let sample_data = [
            [0, 1, 1],
            [1, 2, 1],
            [2, 3, 2],
            [0, 3, 2],
            [0, 4, 3],
            [3, 4, 3],
            [1, 4, 6],
        ];
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                5,
                sample_data.iter().map(|edge| edge.to_vec()).collect()
            ),
            [vec![0, 1], vec![2, 3, 4, 5]]
        );
    }
    #[test]
    fn cycle_case() {
        let sample_data = [[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]];
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                4,
                sample_data.iter().map(|edge| edge.to_vec()).collect()
            ),
            [vec![], vec![0, 1, 2, 3]]
        );
    }
    #[test]
    fn critical_case() {
        let sample_data = [
            [0, 1, 1],
            [1, 2, 1],
            [0, 2, 1],
            [2, 3, 4],
            [3, 4, 2],
            [3, 5, 2],
            [4, 5, 2],
        ];
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                6,
                sample_data.iter().map(|edge| edge.to_vec()).collect()
            ),
            [vec![3], vec![0, 1, 2, 4, 5, 6]]
        );
    }
    #[test]
    fn non_critical_case() {
        let sample_data = [
            [0, 1, 1],
            [0, 3, 1],
            [0, 2, 1],
            [1, 2, 1],
            [1, 3, 1],
            [2, 3, 1],
        ];
        assert_eq!(
            find_critical_and_pseudo_critical_edges(
                4,
                sample_data.iter().map(|edge| edge.to_vec()).collect()
            ),
            [vec![], vec![0, 1, 2, 3, 4, 5]]
        );
    }
}
