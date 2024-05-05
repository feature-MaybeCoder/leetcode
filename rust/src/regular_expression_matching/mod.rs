use std::char;

enum Matches {
    Char(char),
    ManyChar(char),
    ManyAnyChar,
    AnyChar,
}
type Cache = [[Option<bool>; 21]; 21];
fn dfs(
    s_char: &Vec<char>,
    mut s_idx: usize,
    stack: &Vec<Matches>,
    mut stack_idx: usize,
    cache: &mut Cache,
) -> bool {
    if s_idx >= s_char.len() && stack_idx >= stack.len() {
        return true;
    }
    if s_idx >= s_char.len() {
        while stack_idx < stack.len() {
            match stack[stack_idx] {
                Matches::ManyChar(_) => stack_idx += 1,
                Matches::ManyAnyChar => stack_idx += 1,
                _ => return false,
            };
        }
        return true;
    }
    if stack_idx >= stack.len() {
        return false;
    }
    if cache[s_idx][stack_idx].is_some() {
        return cache[s_idx][stack_idx].unwrap();
    }
    let res = match stack[stack_idx] {
        Matches::AnyChar => dfs(s_char, s_idx + 1, stack, stack_idx + 1, cache),
        Matches::ManyChar(char) => {
            let mut next_match = false;
            while s_idx < s_char.len() && s_char[s_idx] == char {
                next_match = dfs(s_char, s_idx, stack, stack_idx + 1, cache);
                if next_match {
                    break;
                }
                s_idx += 1;
            }
            next_match || dfs(s_char, s_idx, stack, stack_idx + 1, cache)
        }
        Matches::Char(char) => {
            if s_char[s_idx] != char {
                false
            } else {
                dfs(s_char, s_idx + 1, stack, stack_idx + 1, cache)
            }
        }
        Matches::ManyAnyChar => {
            let mut res = false;
            while s_idx <= s_char.len() {
                res = dfs(s_char, s_idx, stack, stack_idx + 1, cache);
                if res {
                    break;
                }
                s_idx += 1;
            }
            res
        }
    };
    cache[s_idx][stack_idx] = Some(res);
    return res;
}
pub fn is_match(s: String, p: String) -> bool {
    let mut stack: Vec<Matches> = Vec::with_capacity(p.len());
    let chars: Vec<char> = p.chars().collect();
    let mut idx = chars.len() - 1;
    while idx >= 0 {
        match chars[idx] {
            '*' => {
                idx -= 1;
                let nxt = chars[idx];
                match nxt {
                    '.' => stack.push(Matches::ManyAnyChar),
                    _ => stack.push(Matches::ManyChar(nxt)),
                }
            }
            '.' => stack.push(Matches::AnyChar),
            _ => stack.push(Matches::Char(chars[idx])),
        };
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    stack = stack.into_iter().rev().collect();
    let chars: Vec<char> = s.chars().collect();
    let mut cache: Cache = [[None; 21]; 21];
    return dfs(&chars, 0, &stack, 0, &mut cache);
}
#[cfg(test)]
mod test {
    use super::is_match;

    #[test]
    fn base_case() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
    }
    #[test]
    fn any_char() {
        assert_eq!(is_match("aba".to_string(), "a.a".to_string()), true);
    }
    #[test]
    fn any_many_char() {
        assert_eq!(
            is_match("ababerberber".to_string(), "a.*".to_string()),
            true
        );
    }
    #[test]
    fn many_char() {
        assert_eq!(
            is_match("abccccdddd".to_string(), "abc*d*".to_string()),
            true
        );
    }
    #[test]
    fn zero_many_match() {
        assert_eq!(is_match("aa".to_string(), "a*b".to_string()), false);
    }
    #[test]
    fn many_remain_case() {
        assert_eq!(is_match("aa".to_string(), "aab*".to_string()), true);
    }
}
