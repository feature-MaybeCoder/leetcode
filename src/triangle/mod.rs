pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut row: Vec<i32> = triangle.last().unwrap().clone();
    let len = triangle.len()-1;
    for x in (0..len).rev() {
        
        row = triangle[x].iter().enumerate().map(|(idx, &num)|{
            num + std::cmp::min(row[idx], row[idx + 1])
        }).collect();
     
    }
    return row[0];
}
#[cfg(test)]
mod tests {
    use crate::triangle::minimum_total;

    #[test]
    fn basic_triangle_test() {
        let sample = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(minimum_total(sample), 11)
    }
}
