use std::{collections::HashMap};


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
pub fn maximal_square_dfs(matrix: Vec<Vec<char>>) -> i32 {
    let width = matrix[0].len();
    let height = matrix.len();
    let mut cache = HashMap::new();
    dfs(0, 0, height, width, &matrix, &mut cache);
    let max = *cache.values().max().unwrap_or(&0);
    return max * max;
}
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let width = matrix[0].len();
    let height = matrix.len();
    let mut dp = vec![vec![0; width]; height];
    let width_bound = width - 1;
    let height_bound = height - 1;
    let mut max = 0;
    for x in 0..height{
        let mut val = 0;
        if matrix[x][width_bound]=='1'{
            val =1;
        }
        if val ==1 {
            max =1;
        }
        dp[x][width_bound] = val;
    }
    for y in 0..width{
        let mut val = 0;
        if matrix[height_bound][y]=='1'{
            val =1;
        }
        if val ==1 {
            max =1;
        }
        dp[height_bound][y] = val;
    }
    
    for x in (0..height_bound).rev() {
        for y in (0..width_bound).rev() {
            if matrix[x][y] == '0' {
                continue;
            }
            let min = dp[x + 1][y].min(dp[x][y + 1]).min(dp[x + 1][y + 1]);
            dp[x][y] = min + 1;
            max = max.max(min + 1);
        }
    }

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
    #[test]
    fn dp_edge_case() {
        assert_eq!(maximal_square(vec![vec!['1', '1'],]), 1)
    }
}
