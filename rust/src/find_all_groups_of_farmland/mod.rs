fn dfs(
    x: i32,
    y: i32,
    land: &mut Vec<Vec<i32>>,
    height: i32,
    width: i32,
    dirs: &[i32; 5],
) -> (i32, i32) {
    if x < 0 || y < 0 || x >= height || y >= width || land[x as usize][y as usize] == 0 {
        return (-1, -1);
    }
    land[x as usize][y as usize] = 0;
    let mut max = (x, y);

    for idx in 0..4 {
        let dx = x + dirs[idx];
        let dy = y + dirs[idx + 1];
        let res = dfs(dx, dy, land, height, width, dirs);
        if res.0 > max.0 || res.1 > max.1 {
            max = res;
        }
    }

    return max;
}
pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let height = land.len();
    let width = land[0].len();
    let mut res = Vec::with_capacity(height);
    let dirs = [0, 1, 0, -1, 0];
    let height = height as i32;
    let width = width as i32;
    for x in 0..height {
        for y in 0..width {
            if land[x as usize][y as usize] == 0 {
                continue;
            }
            let mut new = vec![x, y];
            let cur = dfs(x, y, &mut land, height, width, &dirs);
            new.push(cur.0);
            new.push(cur.1);
            res.push(new);
        }
    }
    return res;
}
#[cfg(test)]
mod test {
    use super::find_farmland;

    #[test]
    fn base_case() {
        let sample = [[1, 0, 0], [0, 1, 1], [0, 1, 1]];
        assert_eq!(
            find_farmland(sample.iter().map(|item| item.to_vec()).collect()),
            [[0, 0, 0, 0], [1, 1, 2, 2]]
        );
    }
}
