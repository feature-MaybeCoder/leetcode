pub fn my_atoi(s: String) -> i32 {
    let mut is_in_num = false;
    let mut num_started = false;
    let mut num_ended = false;
    let mut is_negative = false;
    let mut num_start: usize = 0;
    let mut num_end: usize = 0;
    let chars_v: Vec<char> = s.chars().collect();
    for (idx, char) in chars_v.iter().enumerate() {
        match (char, is_in_num, num_started) {
            ('-', true, _) => {
                num_end = idx;
                num_ended = true;
                break;
            }
            ('-', false, _) => {
                is_in_num = true;
                is_negative = true;
            }
            (' ', false, _) => {
                continue;
            }
            (' ', true, _) => {
                num_end = idx;
                num_ended = true;
                break;
            }
            ('+', true, _) => {
                num_end = idx;
                num_ended = true;
                break;
            }
            ('+', false, _) => {
                is_in_num = true;
                continue;
            }
            ('0', _, false)=>{
                is_in_num = true;
                continue;
            }
            ('0'..='9', _, false) => {
                is_in_num = true;
                num_started = true;
                num_start = idx;
                continue;
            }
            ('0'..='9', _, _) => {
                continue;
            }
            _ => {
                num_end = idx;
                num_ended = true;
                break;
            }
        }
    }
    if !num_started {
        return 0;
    }
    if !num_ended {
        num_end = s.len();
    }

    let mut devision = 1;
    let mut sum: i32 = 0;
    for (id, idx) in (num_start..num_end).rev().enumerate() {
        let mut digit = chars_v[idx].to_digit(10).unwrap() as i32;
        if is_negative {
            digit *= -1;
        }
        let mult = digit.checked_mul(devision);

        if mult.is_none() {
            if is_negative {
                sum = i32::MIN;
            } else {
                sum = i32::MAX;
            }
            break;
        }
        let add = sum.checked_add(mult.unwrap());

        if add.is_none() {
            if is_negative {
                sum = i32::MIN;
            } else {
                sum = i32::MAX;
            }
            break;
        }
        sum = add.unwrap();
        let nxt = devision.checked_mul(10);
        if nxt.is_none() {
            if id == num_end-num_start-1{
                break;
            }
            if is_negative {
                sum = i32::MIN;
            } else {
                sum = i32::MAX;
            }
            break;
        }
        devision = nxt.unwrap();
    }
    return sum;
}
#[cfg(test)]
mod test {
    use super::my_atoi;

    #[test]
    fn base_case() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }
    #[test]
    fn negative() {
        assert_eq!(my_atoi("-2".to_string()), -2);
    }
    #[test]
    fn custom_front() {
        assert_eq!(my_atoi("word2".to_string()), 0);
    }
    #[test]
    fn front_minus() {
        assert_eq!(my_atoi("- -2".to_string()), 0);
    }
    #[test]
    fn overflow_case() {
        assert_eq!(
            my_atoi("900090000001000000000000000000".to_string()),
            2147483647
        );
    }
    #[test]
    fn negative_overflow_case() {
        assert_eq!(
            my_atoi("-900090000001000000000000000000".to_string()),
            -2147483648
        );
    }
    #[test]
    fn pre_negative_overflow_case() {
        assert_eq!(my_atoi("-2147483647".to_string()), -2147483647);
    }
    #[test]
    fn start_space_zero() {
        assert_eq!(my_atoi("  0000000000012345678".to_string()), 12345678);
    }
    #[test]
    fn start_space_zero_negative() {
        assert_eq!(my_atoi("  -0000000000012345678".to_string()), -12345678);
    }
}
