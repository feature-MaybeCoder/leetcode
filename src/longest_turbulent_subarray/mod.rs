use std::cmp::{self, Ordering};

pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 1;
    let mut max_len = 1;
    let mut prev_operation: Option<Ordering> = None;
    let mut prev_number = arr[left];
    while right < arr.len() {
        let number = arr[right];
        let cmp_res = prev_number.cmp(&number);
        let cur_op = Some(cmp_res);
        if cur_op == prev_operation{
            max_len = max_len.max(right - left);
            left = right-1;
        }
        prev_operation = cur_op;
        if cmp_res == Ordering::Equal {
            max_len = max_len.max(right - left);
            left = right;
            prev_operation = None;
            
        }
        right+=1;
        prev_number = number;
    }
    max_len = max_len.max(right - left);
    max_len as i32
}
#[cfg(test)]
mod test {
    use super::max_turbulence_size;

    #[test]
    fn base_case() {
        assert_eq!(max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
    }
    #[test]
    fn turbulent_case() {
        assert_eq!(max_turbulence_size(vec![0,8,45,88,48,68,28,55,17,24]), 8);
    }
}
