#[derive(PartialEq, Debug)]
enum Operator {
    Minus,
    Plus,
}
fn evaluate(stack: &mut Vec<i32>, operators_stack: &mut Vec<Operator>) -> i32 {
    let mut rhs = stack.pop().unwrap();
    let mut operator = operators_stack.pop().unwrap();
    let mut lhs = stack.pop().unwrap();
    match operator {
        Operator::Plus => rhs + lhs,
        Operator::Minus => lhs - rhs,
    }
}
pub fn calculate(mut s: String) -> i32 {
    // push padding char for last stack evaluation case
    s.push(' ');
    // main stack containing parsed numbers
    let mut stack: Vec<i32> = Vec::with_capacity(s.len());
    // stack containing operators enum
    let mut operators_stack: Vec<Operator> = Vec::with_capacity(s.len());
    // stack containing previus stack length and reverse length when we rollout from parentheses
    let mut p_lens: Vec<(usize, usize)> = Vec::with_capacity(s.len());
    // usize containig start of string slice containing int that we need to parse
    let mut int_start = 0;
    // represents if we in string int slice that we need to parse
    let mut is_in_int_substr: bool = false;
    // represents whether the previous parsed character is an integer, used for reverse functionality
    let mut is_prev_number: bool = false;
    // cur stack length, actual stack length is not correct because parentheses recursion
    let mut len = 0;
    // cur reverse amount, if we have more than zero, we need to reverse next int
    let mut reverse: usize = 0;

    for (index, char) in s.chars().enumerate() {
        if is_in_int_substr
            && (char == '(' || char == ')' || char == '-' || char == '+' || char == ' ')
        {
            len += 1;
            let mut int = s[int_start..index].parse::<i32>().unwrap();
            if reverse > 0 {
                reverse -= 1;
                int *= -1;
            }
            stack.push(int);
            is_in_int_substr = false;
            is_prev_number = true;
        }
        if len >= 2 {
            let mut s = evaluate(&mut stack, &mut operators_stack);
            if reverse > 0 {
                reverse -= 1;
                s *= -1;
            }
            stack.push(s);
            len -= 1;
            is_prev_number = true;
        }
        match char {
            ')' => {
                let t = p_lens.pop().unwrap();
                let old_len = t.0;
                let mut old_rev_len = t.1;
                if len > 0 {
                    is_prev_number = true;
                }
                if old_rev_len > 0 {
                    let reversed = stack.pop().unwrap() * -1;
                    stack.push(reversed);
                    old_rev_len -= 1;
                }
                len += old_len;
                reverse += old_rev_len;
            }
            '(' => {
                p_lens.push((len, reverse));
                is_prev_number = false;
                len = 0;
                reverse = 0;
            }
            '+' => {
                operators_stack.push(Operator::Plus);
                is_prev_number = false
            }
            '-' => {
                if !is_prev_number {
                    reverse += 1;
                } else {
                    operators_stack.push(Operator::Minus);
                }
                is_prev_number = false;
            }
            '0'..='9' => {
                if !is_in_int_substr {
                    int_start = index;
                }
                is_in_int_substr = true
            }
            ' ' => {}
            _ => {
                panic!("string contains unhandled characters")
            }
        }
    }
    let mut res = stack.pop().unwrap();
    if reverse > 0 {
        res *= -1;
    }
    return res;
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        assert_eq!(calculate("1 + 1".to_string()), 2)
    }
    #[test]
    fn reverse_case() {
        assert_eq!(calculate("1 + -1".to_string()), 0)
    }
    #[test]
    fn p_case() {
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23)
    }
    #[test]
    fn spaces_case() {
        assert_eq!(calculate("1-(     -2)".to_string()), 3)
    }
    #[test]
    fn init_reverse_case() {
        assert_eq!(calculate("-2+ 1".to_string()), -1)
    }
    #[test]
    fn p_reverse_case() {
        assert_eq!(calculate("- (3 + (4 + 5))".to_string()), -12)
    }
    #[test]
    fn p_evalute() {
        assert_eq!(calculate("(7)-(0)+(4)".to_string()), 11)
    }
    #[test]
    fn p_reverse_evalute() {
        assert_eq!(calculate("-(7)-(0)+(4)".to_string()), -3)
    }
    #[test]
    fn p_reverse_ii() {
        assert_eq!(calculate("-(3+4)+5".to_string()), -2)
    }
}
