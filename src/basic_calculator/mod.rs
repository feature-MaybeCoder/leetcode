pub fn calculate(s: String) -> i32 {
    let mut stack: Vec<&str> = Vec::with_capacity(s.len());
    let mut int_start = 0;
    let mut is_in_int_substr: bool = false;
    let mut len = 0;
    let evaluate = || {

    };
    for (index, char) in s.chars().enumerate() {
        if is_in_int_substr && (char == '(' || char == ')' || char == '-' || char == '+') {
            len += 1;
            stack.push(&s[int_start..index]);
        }
        match char {
            ')' => {}
            '(' => {}
            '+' => {}
            '-' => {}
            '0'..='9' => {
                if !is_in_int_substr {
                    int_start = index;
                }
                is_in_int_substr = true
            }
            _ => {
                panic!("string contains unhandle characters")
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        assert_eq!(calculate("1 + 1".to_string()), 2)
    }
}
