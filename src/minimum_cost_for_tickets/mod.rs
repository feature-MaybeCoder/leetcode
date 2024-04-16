fn get_next_idx(mut idx: usize, mut amount: i32, days: &Vec<i32>) -> usize {
    let bound = days.len() - 1;
    while amount > 0 && idx < days.len() {
        if idx >= bound {
            return idx + 1;
        }
        amount -= days[idx + 1] - days[idx];
        idx += 1;
    }
    return idx;
}
fn dfs(idx: usize, cost: i32, days: &Vec<i32>, costs: &Vec<i32>, cache: &mut [i32; 365]) -> i32 {
    if idx == days.len() {
        return cost;
    }
    if cache[idx] != -1 {
        return cache[idx] + cost;
    }
    let mut min = i32::MAX;

    min = dfs(get_next_idx(idx, 1, &days), costs[0], days, costs, cache);
    min = min.min(dfs(
        get_next_idx(idx, 7, &days),
        costs[1],
        days,
        costs,
        cache,
    ));
    min = min.min(dfs(
        get_next_idx(idx, 30, &days),
        costs[2],
        days,
        costs,
        cache,
    ));
    cache[idx] = min;
    return min + cost;
}
pub fn mincost_tickets_dfs(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let mut cache: [i32; 365] = [-1; 365];

    dfs(0, 0, &days, &costs, &mut cache)
}
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let mut cache: [i32; 366] = [0; 366];
    let vars: [i32; 3] = [1, 7, 30];
    for idx in (0..days.len()).rev() {
        let mut min = i32::MAX;
        for day in 0..3 {
            let cost = costs[day];
            let nxt = get_next_idx(idx, vars[day], &days);
    
            min = min.min(cost + cache[nxt]);
        }
        cache[idx] = min;
    }
    return cache[0];
}
#[cfg(test)]
mod test {
    use super::mincost_tickets;

    #[test]
    fn base_case() {
        assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
    }
}
