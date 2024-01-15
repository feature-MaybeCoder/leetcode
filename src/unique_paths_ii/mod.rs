pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let height = obstacle_grid.len();
    let width = obstacle_grid[0].len();
    let height_edge = height - 1;
    let width_edge = width - 1;

    for x in (0..height).rev() {
        for y in (0..width).rev() {
            if obstacle_grid[x][y] == 1 {
                obstacle_grid[x][y] = 0;
                continue;
            }
            let mut sum = 0;
            if y != width_edge{
                sum += obstacle_grid[x][y + 1];
            }
            if x != height_edge{
                sum += obstacle_grid[x + 1][y];
            }
            if x == height_edge && y == width_edge{
                sum = 1;
            }
            obstacle_grid[x][y] = sum;
        }
    }

    return obstacle_grid[0][0];
}
#[cfg(test)]
mod tests {
    use crate::unique_paths_ii::unique_paths_with_obstacles;

    #[test]
    fn basic_triangle_test() {
        let sample = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(sample), 2);
    }
}
