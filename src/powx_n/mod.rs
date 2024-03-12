pub fn my_pow(x: f64, n: i32) -> f64 {
    let n = n as i64;
    if x ==0_f64{
        return 0_f64
    }
    if n ==0{
        return 1_f64;
    }
    let mut res = x * x;
    let mut cur_n = 2;
    while cur_n < n.abs(){
        res*=res;
        cur_n *= 2;
    }
    while cur_n > n.abs(){
        res =res/x;
        cur_n -=1;
    }
    if n < 0{
        res = 1_f64/res;
    }
    res   
    }
