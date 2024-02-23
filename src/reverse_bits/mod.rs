pub fn reverse_bits(mut x: u32) -> u32 {
    let mut res: u32 = 0;
    
    for _ in 0..32{
        res = res << 1;
        if (x & 1) ==1{
            res = res ^ 1;
        }
        x = x >> 1;   
    }

    res
}
#[cfg(test)]
mod test {
    use super::reverse_bits;

    #[test]
    fn base_case() {
        assert_eq!(reverse_bits(43261596), 964176192);
    }
}
