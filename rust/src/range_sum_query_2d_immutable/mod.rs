struct NumMatrix {
    prefix_cache: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        let height = matrix.len();
        let width = matrix[0].len();
        let mut prefix_cache = vec![vec![0; width]; height];
        for x in 0..height {
            let mut row_sum = 0;
            for y in 0..width {
                let val = matrix[x][y];
                row_sum += val;
                let mut cached_value = row_sum;
                if x > 0 {
                    cached_value += matrix[x - 1][y];
                    matrix[x][y] += matrix[x - 1][y];
                    if y > 0 {
                        cached_value += prefix_cache[x - 1][y - 1];
                    }
                }
                prefix_cache[x][y] = cached_value;
            }
        }
        NumMatrix { prefix_cache }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        let mut sum = self.prefix_cache[row2][col2];
        if row1 > 0 && col1 > 0 {
            sum -= self.prefix_cache[row1 - 1][col2];
            sum -= self.prefix_cache[row2][col1 - 1] - self.prefix_cache[row1 - 1][col1 - 1];
        } else if row1 > 0 {
            sum -= self.prefix_cache[row1 - 1][col2];
        } else if col1 > 0 {
            sum -= self.prefix_cache[row2][col1 - 1];
        }

        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
#[cfg(test)]
mod test {
    use super::NumMatrix;

    #[test]
    fn base_case() {
        let matrix_sample = [[0, 1, 2, 3], [0, 1, 2, 3], [0, 1, 2, 3], [0, 1, 2, 3]];
        let matrix = NumMatrix::new(
            matrix_sample
                .iter()
                .map(|item| item.to_vec())
                .collect::<Vec<_>>(),
        );

        assert_eq!(matrix.sum_region(0, 0, 3, 3), 24);
    }
    #[test]
    fn differ_case() {
        let matrix_sample = [
            [3, 0, 1, 4, 2],
            [5, 6, 3, 2, 1],
            [1, 2, 0, 1, 5],
            [4, 1, 0, 1, 7],
            [1, 0, 3, 0, 5],
        ];
        let matrix = NumMatrix::new(
            matrix_sample
                .iter()
                .map(|item| item.to_vec())
                .collect::<Vec<_>>(),
        );

        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
    }
}
