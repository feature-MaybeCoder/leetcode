use std::collections::HashMap;
fn dfs(
    node: usize,
    adj: &mut Vec<Vec<usize>>,
    visited_am: usize,
    visited_target: usize,
    cur_path: &mut Vec<usize>,
) -> bool {
    cur_path.push(node);
    // if we use all the edges, we found most optimal itinerary in case of lexecal order
    if visited_am == visited_target {
        return true;
    }
    for idx in 0..adj[node].len() {
        // if prev checked node is the same, we dont need to check it
        if idx != 0 && adj[node][idx] == adj[node][idx-1] {
            continue;   
        }
        // we need to mark used edge as visited, so we cant use the same edge forward in recursion
        let child = adj[node].remove(idx);
        if dfs(
            child,
            adj,
            visited_am + 1,
            visited_target,
            cur_path
        ) {
            return true;
        }
        // if we didnt find needed path, return edge in adj list
        adj[node].insert(idx, child);
    }

    cur_path.pop();
    return false;
}
pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
    // sort tickets, so first full paths will be optimal
    tickets.sort_by(|item, item_2| item[1].cmp(&item_2[1]));
    let edges_am = tickets.len();
    // map ticket to it idx, so adj list will be faster
    let mut ticket_to_idx_map: HashMap<String, usize> = HashMap::new();
    // push ticket so we can recreate path back to strings
    let mut idx_to_ticket_map: Vec<String> = Vec::new();
    // path containing indeses of tickets
    let mut path: Vec<usize> = Vec::with_capacity(edges_am);
    // tickets will be always have JFK ticket, so push it as root
    let root = "JFK".to_string();
    ticket_to_idx_map.insert(root.clone(), 0);
    idx_to_ticket_map.push(root);
    let mut amount = 1;
    // determine index of each possible ticket
    for ticket in tickets.iter() {
        if !ticket_to_idx_map.contains_key(&ticket[0]) {
            ticket_to_idx_map.insert(ticket[0].clone(), amount);
            idx_to_ticket_map.push(ticket[0].clone());
            amount += 1;
        }
        if !ticket_to_idx_map.contains_key(&ticket[1]) {
            ticket_to_idx_map.insert(ticket[1].clone(), amount);
            idx_to_ticket_map.push(ticket[1].clone());
            amount += 1;
        }
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); amount];
    // create adj list using founded insses
    for ticket in tickets.iter() {
        let from = *ticket_to_idx_map.get(&ticket[0]).unwrap();
        let to = *ticket_to_idx_map.get(&ticket[1]).unwrap();
        adj[from].push(to);
    }
    // run recursion untill first full path match
    dfs(0, &mut adj, 0, edges_am, &mut path);

    // recreate string path using indesses
    return path
        .into_iter()
        .map(|idx| idx_to_ticket_map[idx].clone())
        .collect();
}
#[cfg(test)]
mod test {
    use super::find_itinerary;

    #[test]
    fn base_case() {
        let sample = [
            ["MUC", "LHR"],
            ["JFK", "MUC"],
            ["SFO", "SJC"],
            ["LHR", "SFO"],
        ];
        assert_eq!(
            find_itinerary(
                sample
                    .into_iter()
                    .map(|node| [node[0].to_string(), node[1].to_string()].to_vec())
                    .collect()
            ),
            ["JFK", "MUC", "LHR", "SFO", "SJC"]
        );
    }
    #[test]
    fn loop_case() {
        let sample = [
            ["JFK", "SFO"],
            ["JFK", "ATL"],
            ["SFO", "ATL"],
            ["ATL", "JFK"],
            ["ATL", "SFO"],
        ];
        assert_eq!(
            find_itinerary(
                sample
                    .into_iter()
                    .map(|node| [node[0].to_string(), node[1].to_string()].to_vec())
                    .collect()
            ),
            ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
        );
    }
}
