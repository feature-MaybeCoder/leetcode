pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut weights: Vec<i32> = Vec::with_capacity(ratings.len());

    for (index, val) in ratings.iter().enumerate() {
        weights.push(0);
    }
    let right_border = ratings.len() - 1;
    let mut sum = 0;

    for index in (0..ratings.len()).rev() {
        let value = ratings[index];
        let mut is_peek = false;
        let mut max = 0;
        if index > 0 {
            if ratings[index - 1] < value {
                max = max.max(weights[index - 1]);
                is_peek = true;
            }
        }
        if index < right_border {
            if ratings[index + 1] < value {
                max = max.max(weights[index + 1]);
                is_peek = true
            }
        }
        if is_peek {
            weights[index] = max + 1;
            continue;
        }

        weights[index] = 1
    }
    for index in 0..ratings.len() {
        let value = ratings[index];

        let mut is_peek = false;
        let mut max = 0;
        if index > 0 {
            if ratings[index - 1] < value {
                max = max.max(weights[index - 1]);
                is_peek = true;
            }
        }
        if index < right_border {
            if ratings[index + 1] < value {
                max = max.max(weights[index + 1]);
                is_peek = true
            }
        }
        if is_peek {
            sum += max + 1;
            weights[index] = max + 1;
            continue;
        }
        sum += 1;
        weights[index] = 1
    }

    return sum;
}

#[cfg(test)]
mod test {
    use super::candy;

    #[test]
    fn baisc_candy_test() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
    }
    #[test]
    fn incrementing_sequence_test_case() {
        assert_eq!(candy(vec![1, 2, 2, 3, 4]), 9);
    }
    #[test]
    fn large_edge_case() {
        assert_eq!(candy(vec![29, 51, 87, 87, 72, 12]), 12);
    }
}
