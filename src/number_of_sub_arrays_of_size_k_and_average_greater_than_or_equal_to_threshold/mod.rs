pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let k = k as usize;
    let len = arr.len();
    let mut sum = 0;
    let mut num = 0;
    for index in 0..k {
        sum += arr[index];
    }
    let index = k;

    for idx in index..len {
        let avg = sum / k as i32;
        if avg >= threshold {
            num += 1;
        }
        sum -= arr[idx - index];
        sum += arr[idx];
    }
    let avg = sum / k as i32;
    if avg >= threshold {
        num += 1;
    }
    num
}

#[cfg(test)]
mod test {
    use super::num_of_subarrays;

    #[test]
    fn base_case() {
        assert_eq!(num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
    }

    #[test]
    fn edge_case() {
        assert_eq!(
            num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
            6
        );
    }
}
