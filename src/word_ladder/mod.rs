use std::collections::{HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
    let len = word_list.len();

    let mut graph: Vec<Vec<usize>> = Vec::with_capacity(len + 1);
    for _ in 0..len + 1 {
        graph.push(Vec::new());
    }
    let mut end_in_list = false;
    let mut end_pos = 0;
    let mut start_in_list = false;
    let mut start_index = 0;
    for (index, string) in word_list.iter().enumerate() {
        let end_match = string
            .chars()
            .zip(end_word.chars())
            .filter(|(a, b)| a != b)
            .count();
        let start_match = string
            .chars()
            .zip(begin_word.chars())
            .filter(|(a, b)| a != b)
            .count();
        if end_match == 0 {
            end_in_list = true;
            end_pos=index;
        }
        if start_match == 0 {
            start_in_list = true;
            start_index = index;
        }
        for check_index in index..len {
            let unmatched_amount = string
                .chars()
                .zip(word_list[check_index].chars())
                .filter(|(a, b)| a != b)
                .count();
            if unmatched_amount != 1 {
                continue;
            }
            graph[index + 1].push(check_index + 1);
            graph[check_index + 1].push(index + 1);
        }
    }

    if !end_in_list {
        return 0;
    }
    if !start_in_list {
        for (index, string) in word_list.iter().enumerate() {
            let unmatched_amount = string
                .chars()
                .zip(begin_word.chars())
                .filter(|(a, b)| a != b)
                .count();
            if unmatched_amount != 1 {
                continue;
            }
            graph[index + 1].push(0);
            graph[0].push(index + 1);
        }
    }
    

    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut depth = 1;
    end_pos+=1;
    if !start_in_list {
        queue.push_back(start_index);
    } else {
        queue.push_back(start_index + 1);
    }
    while !queue.is_empty() {
        let q_len = queue.len();
        for _ in 0..q_len {
            let node = queue.pop_front().unwrap();
            if node == end_pos {
                return depth;
            }
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            for neighboor in &graph[node] {
                queue.push_back(*neighboor);
            }
        }
        depth += 1;
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::ladder_length;

    #[test]
    fn baisc_graph_test() {
        let string_arr = vec!["hot", "dot", "dog", "lot", "log", "cog"];

        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                string_arr.iter().map(|char| char.to_string()).collect()
            ),
            5
        );
    }
    #[test]
    fn graph_contains_start_edge_case() {
        let string_arr = vec!["hit", "hot", "dot", "dog", "lot", "log", "cog"];

        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                string_arr.iter().map(|char| char.to_string()).collect()
            ),
            5
        );
    }
    #[test]
    fn un_existing_path_case() {
        let string_arr = vec!["hot", "dot", "dog", "lot", "log"];

        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                string_arr.iter().map(|char| char.to_string()).collect()
            ),
            0
        );
    }
    #[test]
    fn last_in_the_middle_edge_case() {
        let string_arr = vec!["hot", "dog", "dot"];

        assert_eq!(
            ladder_length(
                "hot".to_string(),
                "dog".to_string(),
                string_arr.iter().map(|char| char.to_string()).collect()
            ),
            3
        );
    }
}
