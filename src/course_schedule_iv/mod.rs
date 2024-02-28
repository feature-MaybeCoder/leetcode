fn find(
    node: usize,
    parent: usize,
    graph: &Vec<Vec<usize>>,
    cache: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if node == parent{
        return true
    }
    if let Some(cached) = cache[node][parent] {
        return cached;
    }
    for p in &graph[node]{
        let founded = find(*p, parent, graph, cache);
        if founded{
            cache[node][parent] = Some(true);
            return true
        }
    }
    cache[node][parent] = Some(false);
    return false
}
pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let num_courses = num_courses as usize;
    let mut res = vec![false; queries.len()];
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); num_courses];
    let mut cache: Vec<Vec<Option<bool>>> = vec![vec![None; num_courses]; num_courses];

    for req in prerequisites {
        graph[req[1] as usize].push(req[0] as usize);
    }
    for (index, query) in queries.iter().enumerate() {
        let parent = query[0] as usize;
        res[index] = find(query[1] as usize, parent, &graph, &mut cache);
    }
    return res;
}
#[cfg(test)]
mod test {
    use super::check_if_prerequisite;

    #[test]
    fn base_case() {
        let prerequisites = [[1, 2], [1, 0], [2, 0]]
            .iter()
            .map(|req| req.to_vec())
            .collect();
        let queries = [[1, 0], [1, 2]].iter().map(|req| req.to_vec()).collect();
        assert_eq!(
            check_if_prerequisite(3, prerequisites, queries),
            [true, true]
        );
    }
    #[test]
    fn indirect_case() {
        let prerequisites = [[0, 1], [1, 2], [0, 3]]
            .iter()
            .map(|req| req.to_vec())
            .collect();
        let queries = [[0, 2], [1, 3]].iter().map(|req| req.to_vec()).collect();
        assert_eq!(
            check_if_prerequisite(4, prerequisites, queries),
            [true, false]
        );
    }
    #[test]
    fn indirect_case_ii() {
        let prerequisites = [[2, 3], [2, 1], [0, 3], [0, 1]]
            .iter()
            .map(|req| req.to_vec())
            .collect();
        let queries = [[0, 1], [0, 3], [2, 3], [3, 0], [2, 0], [0, 2]]
            .iter()
            .map(|req| req.to_vec())
            .collect();
        assert_eq!(
            check_if_prerequisite(4, prerequisites, queries),
            [true, true, true, false, false, false]
        );
    }
}
