

----

Tags: #leetcode #medium #graph #dijkstra #dp #depth-first-search

----

## Drawing:
[[number-of-restricted-paths-from-first-to-last-node.excalidraw]]

----


## solution explanation:
calculate shortest pathes from last node to 1 node using bfs or dijkstra
run dfs with rules:
if node is visit add visit count
if node has GE weight continue loop (that ensure that we dont have loop node->neighboor->parent)
roll up untill we reach target node, from target return 1
in rolling out add node to hash map
#tip hashMap is more efficient in this problem than array with len of K because we certainly can not visit all nodes, but array would be allocated for all nodes, that a lot of unused memory and time for allocation. 



## last submission:
```rust
use std::collections::{BinaryHeap, HashMap, HashSet};
impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let target = n + 1;
    let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); target as usize];
    let mut weights = vec![0; target as usize];
    for edge in edges {
        let node1 = edge[0] as usize;
        let node2 = edge[1] as usize;
        graph[node1].push((node2, edge[2]));
        graph[node2].push((node1, edge[2]));
    }
    let mut min_heap = BinaryHeap::new();
    

    min_heap.push((0, n as usize));
    // calulate shortest path using djikstra
    while !min_heap.is_empty() {
        let popped = min_heap.pop().unwrap();
        let dist = -popped.0;
        let node = popped.1;

        if weights[node] !=0 {
            continue;
        }
        
        weights[node] = dist;

        for edge in &graph[node] {
            if weights[edge.0]!= 0 {
                continue;
            }
            let new_dist = -(dist + edge.1);
            min_heap.push((new_dist, edge.0));
        }
    }
    let MOD: i32 = i32::pow(10, 9) + 7;
    return dfs(1, &weights, &graph, n as usize, &mut HashMap::new(), &MOD);
}
}
fn dfs(
    node: usize,
    weights: &Vec<i32>,
    graph: &Vec<Vec<(usize, i32)>>,
    target: usize,
    visited: &mut HashMap<usize, i32>,
    MOD: &i32,
) -> i32 {
    if node == target {
        return 1;
    }
    if visited.contains_key(&node) {
        return *visited.get(&node).unwrap();
    }
    let mut pathes = 0;
    let weight = weights[node];
    

    for edge in &graph[node] {
        let neighboor = edge.0;
        if weights[neighboor] >= weight && neighboor != target {
            continue;
        }
        pathes += dfs(neighboor, weights, graph, target, visited, MOD);
        pathes %= MOD;
    }
    visited.insert(node, pathes);
    return pathes;
}
```



