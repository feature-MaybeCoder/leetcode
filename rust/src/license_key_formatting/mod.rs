pub fn license_key_formatting(s: String, k: i32) -> String {
    let k = k as usize;
        let mut res = Vec::with_capacity(s.len());
        let mut amount  = 0;
        for (index, byte) in s.as_bytes().iter().enumerate().rev(){
            let char = *byte as char;
            if char == '-'{
                continue;
            }
            if amount < k{
                res.push(*byte);
                amount+=1;
            }else{
                
                res.push('-' as u8);
                res.push(char.to_uppercase().nth(0).unwrap() as u8);
                amount=1;
                
            }
        }
        res.reverse();
        return String::from_utf8(res).unwrap();
    }
