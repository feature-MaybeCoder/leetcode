use std::iter;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
    if num2.len() > num1.len(){
        return Self::add_strings(num2, num1)
    }
    let mut reman: u32 = 0;
    let mut string = num1.chars().rev()
        .zip(num2.chars().rev().chain(iter::repeat('0')))
        .map(|number| {
            let sum = number.0.to_digit(10).unwrap()  + number.1.to_digit(10).unwrap() + reman;
            reman = 0;
            if sum >= 10 {
                reman = 1;
                return char::from_digit(sum - 10, 10).unwrap();
            }
            return char::from_digit(sum as u32, 10).unwrap();
        })
        .collect::<String>();
        if reman > 0{
            string.push('1');
        }
        return string.chars().rev().collect::<String>()
    }
}
#[cfg(test)]
mod test{
    use super::add_strings;

    #[test]
    fn base_case(){
        assert_eq!(add_strings("11".to_string(), "123".to_string()), "134");
    }
}
