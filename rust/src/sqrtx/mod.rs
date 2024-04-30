pub fn my_sqrt(x: i32) -> i32 {
    let mut left: i32 = 1;
    let mut right = x;
    let mut mid = left.checked_add(right).unwrap_or(i32::MAX) >> 1;
    while right - left > 0 {
        let pow = (mid as i64).checked_mul(mid as i64).unwrap_or(i64::MAX);
        
        if pow > x as i64 {
            right = mid - 1;
            mid = left + right >> 1;
            continue;
        }
        if pow < x as i64 {
            left = mid + 1;
            mid = left + right >> 1;
            continue;
        }
        break;
    }
    if mid * mid > x {
        mid -= 1;
    }
    return mid;
}

#[cfg(test)]
mod test {
    use super::my_sqrt;

    #[test]
    fn base_case() {
        assert_eq!(my_sqrt(8),2);
    }
    #[test]
    fn int_overflow_case() {
        assert_eq!(my_sqrt(2147395599),46339);
    }
}
