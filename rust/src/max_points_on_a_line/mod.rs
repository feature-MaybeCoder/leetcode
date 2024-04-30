use std::collections::HashMap;
#[derive(Debug)]
struct Point {
    is_visited: bool,
}
fn find_diagonal_points(
    start_x: i32,
    start_y: i32,
    start_point: &Point,
    coord_map: &HashMap<(i32, i32), Point>,
    x: i32,
    y: i32,
    height: i32,
    width: i32,
) -> i32 {
    let dirs = [[1, 1], [-1, -1]];
    let mut max_amount = 0;
    println!(
        "{:?}, x: {:?},height: {:?},y: {:?},width: {:?}, start_x: {:?}, start_y: {:?}",
        coord_map, x, height, y, width, start_x, start_y
    );
    for [dx, dy] in &dirs {
        let mut cur_x = start_x + dx;
        let mut cur_y = start_y + dy;

        let mut amount = 1;
        while cur_x <= height && cur_x >= x && cur_y >= y && cur_y <= width {
            println!("{:?},{:?}, find from left to right", cur_x, cur_y);
            if coord_map.contains_key(&(cur_x, cur_y)) {
                amount += 1;
            }
            cur_x += dx;
            cur_y += dy;
        }
        max_amount = max_amount.max(amount);
    }
    let dirs = [[-1, 1], [1, -1]];
    for [dx, dy] in &dirs {
        let mut cur_x = start_x + dx;
        let mut cur_y = start_y + dy;
        let mut amount = 1;
        while cur_x <= height && cur_x >= x && cur_y >= y && cur_y <= width {
            println!("{:?},{:?}, find from right to left", cur_x, cur_y);
            if coord_map.contains_key(&(cur_x, cur_y)) {
                amount += 1;
            }
            cur_x += dx;
            cur_y += dy;
        }
        max_amount = max_amount.max(amount);
    }
    return max_amount;
}
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut x_map: HashMap<i32, i32> = HashMap::with_capacity(len);
    let mut y_map: HashMap<i32, i32> = HashMap::with_capacity(len);
    let mut coord_map: HashMap<(i32, i32), Point> = HashMap::with_capacity(len);

    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_count = 0;

    for point in points {
        let x = point[0];
        let y = point[1];
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        let x_amount = x_map.entry(x).or_insert(0);
        *x_amount += 1;
        let y_amount = y_map.entry(y).or_insert(0);
        *y_amount += 1;
        max_count = max_count.max(*x_amount).max(*y_amount);
        coord_map.insert((x, y), Point { is_visited: false });
    }

    for ((x, y), point) in coord_map.iter() {
        if point.is_visited {
            continue;
        }
        max_count = max_count.max(find_diagonal_points(
            *x, *y, point, &coord_map, min_x, min_y, max_x, max_y,
        ));
    }
    return max_count;
}
#[cfg(test)]
mod test {
    use super::max_points;

    #[test]
    fn base_case() {
        let points = [[1, 1], [2, 2], [3, 3]];
        assert_eq!(
            max_points(points.iter().map(|point| point.to_vec()).collect()),
            3
        );
    }
    #[test]
    fn find_max_on_x() {
        let points = [[0, 1], [0, 2], [0, 4], [2, 1], [2, 2], [2, 4], [2, 5]];
        assert_eq!(
            max_points(points.iter().map(|point| point.to_vec()).collect()),
            4
        );
    }
    #[test]
    fn find_max_on_y() {
        let points = [[1, 1], [2, 1], [3, 1], [0, 3], [1, 3], [4, 3], [5, 3]];
        assert_eq!(
            max_points(points.iter().map(|point| point.to_vec()).collect()),
            4
        );
    }
    #[test]
    fn find_max_on_left_to_right_diagonal() {
        let points = [[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
        assert_eq!(
            max_points(points.iter().map(|point| point.to_vec()).collect()),
            4
        );
    }
    #[test]
    fn fond_bound_diagonal() {
        let points = [[2,3],[1,1]];
        assert_eq!(
            max_points(points.iter().map(|point| point.to_vec()).collect()),
            4
        );
    }
}
