fn build(arr: &[i64; 32]) -> i32 {
    let mut ret = 0;

    for i in (0..32).rev() {
        ret *= 3;
        ret += arr[i];
    }

    ret as i32
}
pub fn single_number(nums: Vec<i32>) -> i32 {
    const SIZE: usize = 32;
    let mut pos = [0 as i64; SIZE];
    let mut neg = [0 as i64; SIZE];
    for num in nums {
        let mut num = num as i64;
        let mut cur = &mut pos;
        if num < 0 {
            num = num * -1;
            cur = &mut neg;
        }
        for index in 0..SIZE {
            cur[index] += num as i64;
            cur[index] %= 3;
            num = (num as f64 / 3_f64).floor() as i64;
            if num == 0{
                break
            }
        }
    }
    if neg.iter().sum::<i64>() > 0 {
        return build(&neg) * -1;
    }
    build(&pos)
}
#[cfg(test)]
mod test {
    use super::single_number;
    #[test]
    fn base_case() {
        assert_eq!(single_number(vec![1,1,1,6]), 6);
    }
    #[test]
    fn negative_duplicates() {
        assert_eq!(single_number(vec![-1, -1, -1, 2, 3, 3, 3]), 2);
    }
    #[test]
    fn negative_overflow() {
        assert_eq!(
            single_number(vec![
                43,
                16,
                45,
                89,
                45,
                -2147483648,
                45,
                2147483646,
                -2147483647,
                -2147483648,
                43,
                2147483647,
                -2147483646,
                -2147483648,
                89,
                -2147483646,
                89,
                -2147483646,
                -2147483647,
                2147483646,
                -2147483647,
                16,
                16,
                2147483646,
                43
            ]),
            2147483647
        );
    }
}
