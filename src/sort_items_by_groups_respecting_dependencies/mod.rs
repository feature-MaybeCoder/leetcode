use std::collections::HashSet;
fn tp_sort(
    node: usize,
    visited_cycle: &mut HashSet<usize>,
    visited: &mut HashSet<usize>,
    visited_cycle_un: &mut HashSet<usize>,
    visited_un: &mut HashSet<usize>,
    adj_list: &Vec<Vec<i32>>,
    group: &Vec<i32>,
    groups: &Vec<Vec<usize>>,
    before_items: &Vec<Vec<i32>>,
    res: &mut Vec<i32>,
    is_in_group: bool,
) -> bool {
    if visited_cycle.contains(&node) {
        return true;
    }
    if visited.contains(&node) {
        return false;
    }

    visited.insert(node);
    visited_cycle.insert(node);

    for neighb in &adj_list[node] {
        let contains_cycle = tp_sort(
            *neighb as usize,
            visited_cycle,
            visited,
            visited_cycle_un,
            visited_un,
            adj_list,
            group,
            groups,
            before_items,
            res,
            is_in_group,
        );
        if contains_cycle {
            return true;
        }
    }
    visited_cycle.remove(&node);
    if is_in_group {
        for grouped_nodes in &groups[node as usize] {
            let contains_cycle = tp_sort(
                *grouped_nodes,
                visited_cycle_un,
                visited_un,
                visited_cycle,
                visited,
                before_items,
                group,
                groups,
                before_items,
                res,
                false,
            );
            if contains_cycle {
                return true;
            }
        }
    } else {
        res.push(node as i32);
    }
    return false;
}
pub fn sort_items(n: i32, m: i32, mut group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(n as usize);
    let mut groups: Vec<Vec<usize>> = Vec::with_capacity(n as usize);
    let mut graph: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
    let mut visited_set: HashSet<usize> = HashSet::with_capacity(n as usize);
    let mut visited_set_cycle: HashSet<usize> = HashSet::with_capacity(n as usize);
    let mut visited_set_g: HashSet<usize> = HashSet::with_capacity(n as usize);
    let mut visited_set_cycle_g: HashSet<usize> = HashSet::with_capacity(n as usize);
    let mut last_added = m as usize;

    for _ in 0..n + 1 {
        groups.push(Vec::with_capacity(n as usize));
        graph.push(Vec::new());
    }
    for node in 0..n {
        let node = node as usize;
        let node_group = group[node];
        if node_group == -1 {
            groups[last_added].push(node);
            group[node] = last_added as i32;
            last_added += 1;
            continue;
        }
        groups[node_group as usize].push(node);
    }

    for node in 0..n {
        let node = node as usize;
        let node_group = group[node];
        let before_els = &before_items[node];
        for index in 0..before_els.len() {
            let before_node = before_els[index] as usize;
            let before_node_group = group[before_node];
            if before_node_group == node_group {
                continue;
            }
            graph[node_group as usize].push(before_node_group);
        }
    }

    for node in 0..groups.len() {
        let contains_cycle = tp_sort(
            node as usize,
            &mut visited_set_cycle_g,
            &mut visited_set_g,
            &mut visited_set,
            &mut visited_set_cycle,
            &graph,
            &group,
            &groups,
            &before_items,
            &mut res,
            true,
        );
        if contains_cycle {
            return vec![];
        }
    }
    return res;
}
#[cfg(test)]
mod test {
    use std::fs;

    use super::sort_items;

    #[test]
    fn base_case() {
        let group_sample = [-1, -1, 1, 0, 0, 1, 0, -1].to_vec();
        let before_sample: Vec<Vec<i32>> = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3, 6],
            vec![],
            vec![],
            vec![],
        ];
        
        assert_eq!(
            sort_items(8, 2, group_sample, before_sample),
            [6, 3, 4, 1, 5, 2, 0, 7]
        );
    }
    #[test]
    fn cycle_test() {
        let group_sample = [-1, -1, 1, 0, 0, 1, 0, -1].to_vec();
        let before_sample: Vec<Vec<i32>> = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3],
            vec![],
            vec![4],
            vec![],
        ];

        // assert_eq!(sort_items(8, 2, group_sample, before_sample), []);
    }
    #[test]
    fn group_break_case() {
        let group_sample = [0, 0, 2, 1, 0].to_vec();
        let before_sample: Vec<Vec<i32>> = vec![vec![3], vec![], vec![], vec![], vec![1, 3, 2]];

        assert_eq!(
            sort_items(5, 3, group_sample, before_sample),
            [3, 2, 0, 1, 4]
        );
    }
    #[test]
    fn group_overflow_test() {
        let group_sample = [2, 0, -1, 3, 0].to_vec();
        let before_sample: Vec<Vec<i32>> = vec![vec![2, 1, 3], vec![2, 4], vec![], vec![], vec![]];

        assert_eq!(
            sort_items(5, 5, group_sample, before_sample),
            [3, 2, 0, 1, 4]
        );
    }
    #[test]
    fn group_join_case() {
        let group_sample = [-1, -1, 1, 0, 0, 1, 0, -1].to_vec();
        let before_sample: Vec<Vec<i32>> = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3, 6],
            vec![],
            vec![7],
            vec![],
        ];

        assert_eq!(
            sort_items(8, 2, group_sample, before_sample),
            [3, 2, 0, 1, 4]
        );
    }
    #[test]
    fn large_data_test_case() {
        let file = fs::File::open(
            "./src/sort_items_by_groups_respecting_dependencies/large_test_case_before.json",
        )
        .expect("file should open read only");
        let before: Vec<Vec<i32>> =
            serde_json::from_reader(file).expect("file should be proper JSON");
        let file = fs::File::open(
            "./src/sort_items_by_groups_respecting_dependencies/large_test_case_group.json",
        )
        .expect("file should open read only");
        let groups: Vec<i32> = serde_json::from_reader(file).expect("file should be proper JSON");
        
        sort_items(30000, 13931, groups, before);
    }
}
