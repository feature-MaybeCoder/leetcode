

----

Tags: #leetcode #dp #string #medium

----

## Drawing:
[[word-break.excalidraw]]

----


## solution explanation:
decision tree with width of dicts, if word slice of index..end start with dict word, run dfs, add res to cache by index

## last submission:
Rust

```rust
use std::collections::{HashMap};
fn dfs_word(
    s: &String,
    index: usize,
    cache: &mut HashMap<usize, bool>,
    dict: &Vec<String>,
) -> bool {
    
    if cache.contains_key(&index) {
        return *cache.get(&index).unwrap();
    }

    let mut res = false;
    for str in dict{
        if s[index..].starts_with(str){
            let newLen = index + str.len();
            if newLen == s.len(){
                res = true;
                break
            }
            if dfs_word(s, newLen, cache, dict){
                res = true
            }
        };
    }
    cache.insert(index, res);
    return res
}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut cache: HashMap<usize, bool> = HashMap::new();
        return dfs_word(&s, 0, &mut cache, &word_dict);
    }
}
```



