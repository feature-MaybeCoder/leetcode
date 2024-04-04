use std::{cmp::Reverse, collections::BinaryHeap};

fn manhattan_distance(pt1: &Vec<i32>, pt2: &Vec<i32>)->i32{
    return (pt1[0].abs_diff(pt2[0]) + pt1[1].abs_diff(pt2[1])) as i32;
}
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut visit: Vec<bool> = vec![false; len];
    let mut frontier: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::with_capacity(len);
    frontier.push(Reverse((0, 0)));
    let mut sum = 0;
    let mut visited_am = 0;
    while !frontier.is_empty() && visited_am < len {
        let (weight, index) = frontier.pop().unwrap().0;
        if visit[index]{
            continue;
        }
        visited_am+=1;
        sum+=weight;
        visit[index] = true;
        for neighboor in 0..len{
            if neighboor == index || visit[neighboor]{
                continue;
            }
            frontier.push(Reverse((manhattan_distance(&points[neighboor], &points[index]), neighboor)));
            
        }
    }
    return sum
}
#[cfg(test)]
mod test {
    use super::min_cost_connect_points;

    #[test]
    fn base_case() {
        let sample = [[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]];
        assert_eq!(
            min_cost_connect_points(sample.iter().map(|point| point.to_vec()).collect()),
            20
        );
    }
    #[test]
    fn negative_case() {
        let sample =[[3,12],[-2,5],[-4,1]];
        assert_eq!(
            min_cost_connect_points(sample.iter().map(|point| point.to_vec()).collect()),
            18
        );
    }
    #[test]
    fn small_case() {
        let sample =[[0,0],[1,1],[1,0],[-1,1]];
        assert_eq!(
            min_cost_connect_points(sample.iter().map(|point| point.to_vec()).collect()),
            4
        );
    }
}
