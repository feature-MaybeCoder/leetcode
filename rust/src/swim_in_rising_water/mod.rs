use std::collections::BinaryHeap;

pub fn swim_in_water(mut grid: Vec<Vec<i32>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    if height == 1 && width == 1 {
        return grid[0][0];
    }
    let mut quque: BinaryHeap<(i32, i32, i32)> = BinaryHeap::with_capacity(height * width);
    let height = height as i32;
    let width = width as i32;
    let height_bound = height - 1;
    let width_bound = width - 1;
    let dirs = [0, 1, 0, -1, 0];
    quque.push((-grid[0][0], 0, 0));

    while !quque.is_empty() {
        let (mut max, x, y) = quque.pop().unwrap();
        max *= -1;
        for x_idx in 0..4 {
            let dx = dirs[x_idx];
            let dy = dirs[x_idx + 1];
            let next_x = x + dx;
            let next_y = y + dy;
            if next_x < 0
                || next_x >= height
                || next_y < 0
                || next_y >= width
                || grid[next_x as usize][next_y as usize] < 0
            {
                continue;
            }
            if next_x == height_bound && next_y == width_bound {
                return max.max(grid[next_x as usize][next_y as usize]);
            }
            quque.push((
                -(max.max(grid[next_x as usize][next_y as usize])),
                next_x,
                next_y,
            ));
            grid[next_x as usize][next_y as usize] = -1;
        }
    }
    return -1;
}
#[cfg(test)]
mod test {
    use super::swim_in_water;

    #[test]
    fn base_case() {
        let sample = [
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6],
        ];
        assert_eq!(
            swim_in_water(sample.iter().map(|item| item.to_vec()).collect()),
            16
        );
    }
    #[test]
    fn init_max_case() {
        let sample = [[3, 2], [0, 1]];
        assert_eq!(
            swim_in_water(sample.iter().map(|item| item.to_vec()).collect()),
            3
        );
    }
}
