pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    
    let len = triangle.len()-1;
    for x in (0..len).rev() {
        for y in 0..triangle[x].len(){
            triangle[x][y] = triangle[x][y]+ std::cmp::min(triangle[x+1][y],triangle[x+1][y+1]);
        }
       
     
    }
    return triangle[0][0];
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
