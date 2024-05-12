pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let height = grid.len()-2;
    let width = grid[0].len()-2;
    let mut res = vec![vec![0; width]; height];
    for x in 0..height{
        for y in 0..width{
            let mut max = i32::MIN;
            for x_c in x..(x+3){
                for y_c in y..(y+3){
                    max = max.max(grid[x_c][y_c]);       
                }   
            }
            res[x][y] = max;
        }
    }
    return res;
}
#[cfg(test)]
mod test {
    use super::largest_local;

    #[test]
    fn base_case() {
        let sample = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]];
        assert_eq!(largest_local(sample.iter().map(|row|row.to_vec()).collect()), [[9,9],[8,6]]);
    }
}
