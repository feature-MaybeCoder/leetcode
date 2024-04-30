pub fn trailing_zeroes(n: i32) -> i32 {
    let mut zeros = 0;
    while n >= 5 {
        n = n / 5;
        zeros += n;
    }
    return zeros;
}
