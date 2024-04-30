use std::collections::{HashMap, HashSet, VecDeque};

pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    let mut keyes = ['a', 'b', 'c', 'd', 'e', 'f'];
    let mut locks = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut start_x: Option<usize> = None;
    let mut start_y: Option<usize> = None;
    let mut min_map: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let mut graph: Vec<Vec<char>> = Vec::new();
    let mut keys_amount = 0;
    for (x, string) in grid.iter().enumerate() {
        graph.push(string.chars().collect());
        for (y, char) in string.chars().enumerate() {
            if char == '@' {
                start_x = Some(x);
                start_y = Some(y);
                continue;
            }
            if char.is_lowercase() {
                keys_amount += 1;
            }
        }
    }

    let mut queue: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();
    queue.push_back((start_x.unwrap() as i32, start_y.unwrap() as i32, 0, 0));
    let height = graph.len() as i32;
    let width = graph[0].len() as i32;
    let dirs = [0, 1, 0, -1, 0];
    while !queue.is_empty() {
        let (x, y, keyes_mask, depth) = queue.pop_front().unwrap();
        let hash_key = (x, y, keyes_mask);

        if keyes_mask == ((1 << keys_amount) - 1) {
            return *min_map.get(&hash_key).unwrap();
        }
        for x_idx in 0..4 {
            let dx = x + dirs[x_idx];
            let dy = y + dirs[x_idx + 1];
            if dx < 0 || dx >= height || dy < 0 || dy >= width {
                continue;
            }
            let mut n_mask = keyes_mask;
            let new_char = graph[dx as usize][dy as usize];
            if new_char == '#' {
                continue;
            }
            if new_char.is_uppercase()
                && (keyes_mask & (1 << locks.iter().position(|k| *k == new_char).unwrap())) == 0
            {
                continue;
            }
            if new_char.is_lowercase() {
                n_mask |= 1 << keyes.iter().position(|c| *c == new_char).unwrap();
            }
            let cache_key = (dx, dy, n_mask);
            if min_map.contains_key(&cache_key) && *min_map.get(&cache_key).unwrap() <= depth + 1 {
                continue;
            }
            min_map.insert(cache_key, depth + 1);
            queue.push_back((dx, dy, n_mask, depth + 1));
        }
    }

    return -1;
}

#[cfg(test)]
mod test {

    use super::shortest_path_all_keys;

    #[test]
    fn baisc_graph_test() {
        let sample = vec!["@.a..", "###.#", "b.A.B"];

        assert_eq!(
            shortest_path_all_keys(sample.iter().map(|char| char.to_string()).collect()),
            8
        );
    }
    #[test]
    fn lock_edge_case() {
        let sample = vec![
            "..#....##.",
            "....d.#.D#",
            "#...#.c...",
            "..##.#..a.",
            "...#....##",
            "#....b....",
            ".#..#.....",
            "..........",
            ".#..##..A.",
            ".B..C.#..@",
        ];

        assert_eq!(
            shortest_path_all_keys(sample.iter().map(|char| char.to_string()).collect()),
            19
        );
    }
    #[test]
    fn lock_route_edge_case() {
        let sample = vec!["b", "A", "a", "@", "B"];

        assert_eq!(
            shortest_path_all_keys(sample.iter().map(|char| char.to_string()).collect()),
            3
        );
    }
    #[test]
    fn time_limit_case() {
        let sample = vec!["#..#.#.#..#.#.#.....#......#..",".#.......#....#A.....#.#......","#....#.....#.........#........","...#.#.........#..@....#....#.",".#.#.##...#.........##....#..#","..........#..#..###....##..#.#",".......#......#...#...#.....c#",".#...#.##......#...#.###...#..","..........##...#.......#......","#...#.........a#....#.#.##....","..#..#...#...#..#....#.....##.","..........#...#.##............","...#....#..#.........#..D.....","....#E.#....##................","...........##.#.......#.#....#","...#..#...#.#............#e...","..#####....#.#...........##..#","##......##......#.#...#..#.#..",".#F.......#..##.......#....#..","............#....#..#..#...#..",".............#...#f...#..##...","....#..#...##.........#..#..#.",".....#.....##.###..##.#......#",".#..#.#...#.....#........###..",".....#.#...#...#.....#.....#..","##.....#....B.....#..#b.......",".####....##..#.##..d.#......#.","..#.....#....##........##...##","...#...#...C..#..#....#.......","#.....##.....#.#......#......."];

        assert_eq!(
            shortest_path_all_keys(sample.iter().map(|char| char.to_string()).collect()),
            3
        );
    }
}
