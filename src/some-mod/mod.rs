pub fn random() -> i32{
    return 1;
}


#[cfg(test)]
mod tests {
    use crate::some_mod::random;

    #[test]
    fn it_works() {
        assert_eq!(random(), 1);
    }
}