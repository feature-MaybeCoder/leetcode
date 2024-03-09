use std::collections::{HashMap, HashSet};
#[derive(Debug, Default)]
struct WordNode {
    chars: [Children; 26],
    end: Option<usize>,
    indeses: Vec<i32>,
}
type Children = Option<Box<WordNode>>;
impl WordNode {
    fn contains_key(&self, key: u8) -> bool {
        let i = key as usize - 'a' as usize;
        return self.chars[i].is_some();
    }
    fn get(&self, key: u8) -> &Children {
        let i = key as usize - 'a' as usize;
        return &self.chars[i];
    }
    fn get_mut(&mut self, key: char) -> &mut Children {
        let i = key as usize - 'a' as usize;
        return &mut self.chars[i];
    }
    fn delete(&mut self, key: char) {
        let i = key as usize - 'a' as usize;
        self.chars[i] = None;
    }
    fn get_trie(&self, word: &str) -> Option<&Self> {
        let mut node = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if let Some(child) = &node.chars[idx] {
                node = child;
            } else {
                return None;
            }
        }
        return Some(node);
    }
}
struct WordFilterTree {
    root_pref: Children,
    root_suf: Children,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilterTree {
    fn new(words: Vec<String>) -> Self {
        let mut root_pref: Children = None;
        let mut root_suf: Children = None;
        for (index, word) in words.iter().enumerate() {
            let mut cur_pref = &mut root_pref;
            let mut cur_suf = &mut root_suf;
            for (char_pref, char_suf) in word.chars().zip(word.chars().rev()) {
                let next_pref = cur_pref.get_or_insert_with(Box::default);
                next_pref.indeses.push(index as i32);
                cur_pref = next_pref.get_mut(char_pref);

                let next_suf = cur_suf.get_or_insert_with(Box::default);
                next_suf.indeses.push(index as i32);
                cur_suf = next_suf.get_mut(char_suf);
            }
            let last_pref = cur_pref.get_or_insert_with(Box::default);
            last_pref.indeses.push(index as i32);
            last_pref.end = Some(index);
            let last_suf = cur_suf.get_or_insert_with(Box::default);
            last_suf.indeses.push(index as i32);
            last_suf.end = Some(index);
        }
        Self {
            root_pref: root_pref,
            root_suf: root_suf,
        }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        if self.root_suf.is_none() || self.root_pref.is_none() {
            return 0;
        }
        let pref_res = self.root_pref.as_ref().unwrap().get_trie(&pref);
        let suf_res = self
            .root_suf
            .as_ref()
            .unwrap()
            .get_trie(&suff.chars().rev().collect::<String>());

        if pref_res.is_none() || suf_res.is_none() {
            return -1;
        }
        let pref_res = &pref_res.unwrap().indeses;
        let suf_res = &suf_res.unwrap().indeses;
        
        let mut left = pref_res.len() as i32 - 1;
        let mut right = suf_res.len() as i32 - 1;
        while left >= 0 && right >= 0 {
            if pref_res[left as usize] == suf_res[right as usize] {
                return pref_res[left as usize];
            } else if pref_res[left as usize] > suf_res[right as usize] {
                left -= 1;
            } else {
                right -= 1;
            }
        }
        return -1;
    }
}
struct WordFilter {
    map: HashMap<String, i32>,
}
impl WordFilter {
    fn get_hash_key(pref: &str, suff: &str) -> String {
        let mut key = String::with_capacity(pref.len() + suff.len() + 1);
        key.push_str(pref);
        key.push(' ');
        key.push_str(suff);
        return key;
    }
    fn new(words: Vec<String>) -> Self {
        let mut map = HashMap::with_capacity(words.len());
        for (index, word) in words.iter().enumerate() {
            for i in 0..word.len() {
                for j in 0..word.len() {
                    let key = Self::get_hash_key(&word[..=i], &word[j..]);
                    map.insert(key, index as i32);
                }
            }
        }
        return Self { map: map };
    }
    fn f(&self, pref: String, suff: String) -> i32 {
        let key = Self::get_hash_key(&pref, &suff);
        return *self.map.get(&key).unwrap_or(&-1);
    }
}

#[cfg(test)]
mod test {
    use super::{WordFilter, WordFilterTree};

    #[test]
    fn base_case() {
        let words = vec!["apple", "aplep", "asomewolerdle"];
        let filter = WordFilter::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("a".to_string(), "e".to_string()), 2);
    }
    #[test]
    fn should_work_with_single_char() {
        let words = vec!["a"];
        let filter = WordFilter::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("a".to_string(), "a".to_string()), 0);
    }
    #[test]
    fn ovelayed_pref_suf() {
        let words = vec!["abc", ""];
        let filter = WordFilter::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("ab".to_string(), "bc".to_string()), 0);
    }
    #[test]
    fn ovelayed_pref_suf_ii() {
        let words = vec!["abba"];
        let filter = WordFilter::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("ab".to_string(), "ba".to_string()), 0);
    }

    #[test]
    fn base_case_tree() {
        let words = vec!["apple", "aplep", "asomewolerdle"];
        let filter = WordFilterTree::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("a".to_string(), "e".to_string()), 2);
    }
    #[test]
    fn should_work_with_single_char_tree() {
        let words = vec!["a"];
        let filter = WordFilterTree::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("a".to_string(), "a".to_string()), 0);
    }
    #[test]
    fn ovelayed_pref_suf_tree() {
        let words = vec!["abc", ""];
        let filter = WordFilterTree::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("ab".to_string(), "bc".to_string()), 0);
    }
    #[test]
    fn ovelayed_pref_suf_ii_tree() {
        let words = vec!["abba"];
        let filter = WordFilterTree::new(words.iter().map(|item| item.to_string()).collect());
        assert_eq!(filter.f("ab".to_string(), "ba".to_string()), 0);
    }
}
