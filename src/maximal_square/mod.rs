use std::collections::HashMap;

pub fn dfs(
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    matrix: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if x >= height || y >= width {
        return 0;
    }
    let cache_key = (x, y);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    
    let max = dfs(x + 1, y, height, width, matrix, cache)
        .min(dfs(x, y + 1, height, width, matrix, cache))
        .min(dfs(x + 1, y + 1, height, width, matrix, cache))
        + 1;
    if matrix[x][y] == '0' {
        cache.insert(cache_key, 0);
        return 0;
    }
    cache.insert(cache_key, max);
    return max;
}
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let width = matrix[0].len();
    let height = matrix.len();
    let mut cache =  HashMap::new();
    dfs(0, 0, height, width, &matrix, &mut cache);
    let max = *cache.values().max().unwrap_or(&0);
    return max * max;
}

#[cfg(test)]
mod test {
    use crate::maximal_square::maximal_square;

    #[test]
    fn base_test_case() {
        assert_eq!(
            maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        )
    }
    #[test]
    fn second_base_test_case() {
        assert_eq!(maximal_square(vec![vec!['0', '1'], vec!['1', '0']]), 1)
    }
    #[test]
    fn edge_case() {
        assert_eq!(
            maximal_square(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['0', '0', '1', '1', '1']
            ]),
            16
        )
    }
}
