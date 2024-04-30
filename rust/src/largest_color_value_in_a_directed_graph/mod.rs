use std::collections::HashMap;
fn add_caches(
    node: usize,
    neigb: usize,
    cache: &mut Vec<HashMap<u8, i32>>,
    cur_b: u8,
    unique_b: &Vec<u8>,
) -> i32 {
    let mut max = 0;

    for byte in unique_b {
        let mut val = *cache[neigb].get(byte).unwrap();
        if *byte == cur_b {
            val += 1;
        }
        if val > cache[node][byte] {
            cache[node].insert(*byte, val);
        }
        max = max.max(val)
    }
    return max;
}
fn dfs(
    node: usize,
    visited: &mut Vec<bool>,
    visited_cycle: &mut Vec<bool>,
    colors_b: &Vec<u8>,
    cache: &mut Vec<HashMap<u8, i32>>,
    adj_list: &Vec<Vec<usize>>,
    unique_b: &Vec<u8>,
) -> i32 {
    if visited_cycle[node] {
        return -1;
    }
    if visited[node] == true {
        return *cache[node].get(&('1' as u8)).unwrap()
    }
    visited[node] = true;
    visited_cycle[node] = true;
    cache[node].insert(colors_b[node], 1);
    
    let mut max = 0;
    for neighb in &adj_list[node] {
        let contains_cycle = dfs(
            *neighb,
            visited,
            visited_cycle,
            colors_b,
            cache,
            adj_list,
            unique_b,
        );
        if contains_cycle == -1 {
            return -1;
        }
        max = max.max(add_caches(
            node,
            *neighb,
            cache,
            colors_b[node],
            unique_b,
            
        ))
    }
    visited_cycle[node] = false;
    cache[node].insert('1' as u8, max);
    return max;
}
pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    let len = colors.len();
    let mut visited: Vec<bool> = Vec::with_capacity(len);
    let mut visited_cycle: Vec<bool> = Vec::with_capacity(len);

    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(len);
    let mut cache: Vec<HashMap<_, _>> = Vec::with_capacity(len);
    let colors_b = colors.into_bytes();
    let mut cache_copy_layer: HashMap<_, _> = HashMap::new();
    let mut unique_b: Vec<u8> = Vec::with_capacity(len);
    for index in 0..len {
        let b = colors_b[index];
        if cache_copy_layer.contains_key(&b) {
            continue;
        }
        cache_copy_layer.insert(colors_b[index], 0);
        unique_b.push(b);
    }
    for index in 0..len {
        visited.push(false);
        visited_cycle.push(false);
        cache.push(cache_copy_layer.clone());
        graph.push(Vec::new());
    }

    for index in 0..edges.len() {
        let edge = &edges[index];
        graph[edge[0] as usize].push(edge[1] as usize);
    }

    for node in 0..len {
        if visited[node] {
            continue;
        }
        let contains_cycle = dfs(
            node,
            &mut visited,
            &mut visited_cycle,
            &colors_b,
            &mut cache,
            &graph,
            &unique_b,
        );
        if contains_cycle == -1 {
            return -1;
        }
        max = max.max(contains_cycle);
    }
    return max;
}
#[cfg(test)]
mod test {
    use crate::largest_color_value_in_a_directed_graph::largest_path_value;

    #[test]
    fn base_case() {
        let edges = [[0, 1], [0, 2], [2, 3], [3, 4]];
        assert_eq!(
            largest_path_value(
                "abaca".to_string(),
                edges.iter().map(|array| array.to_vec()).collect()
            ),
            3
        );
    }
}
