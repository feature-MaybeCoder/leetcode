use std::collections::HashMap;
#[derive(Debug)]
struct Node{
    chars: HashMap<char, Node>,
    end: Option<usize>
}
fn dfs(
    board: &mut Vec<Vec<char>>,
    node: &mut Node,
    words: &Vec<String>,
    res: &mut Vec<String>,
    x: usize,
    y: usize,
    dirs: &[i32],
    width: usize,
    height: usize
) {
    if let Some(index) = node.end {
        res.push(words[index].clone());
        node.end = None;
    }
    let temp = board[x][y];
    board[x][y] = '1';
    for d_index in 0..4 {
        let dy = dirs[d_index];
        let dx = dirs[d_index + 1];
        let dx = x as i32 + dx;
        let dy = y as i32 + dy;
        if dx < 0 || dy < 0 {
            continue;
        }
        let dx = dx as usize;
        let dy = dy as usize;
        if dx >= height || dy >= width {
            continue;
        }
        let next = board[dx][dy];
        if node.chars.contains_key(&next){
            dfs(board, node.chars.get_mut(&next).unwrap(), words, res, dx, dy, dirs, width, height);
        }
        
    }
    board[x][y] = temp;
}

pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
let mut res: Vec<String> = Vec::with_capacity(words.len());
let mut root = Node{
    chars: HashMap::new(),
    end: None
};
for (index, word) in words.iter().enumerate(){
    let mut cur = &mut root;
    for char in word.chars(){
        if cur.chars.contains_key(&char){
            cur = cur.chars.get_mut(&char).unwrap();
            continue;
        }
        let new_node = Node{
            chars: HashMap::new(),
            end: None
        };
        cur.chars.insert(char, new_node);
        cur = cur.chars.get_mut(&char).unwrap();
    }
    cur.end = Some(index);
}
let width = board[0].len();
let height = board.len();
let dirs = [0, 1, 0, -1, 0];
for x in 0..height {
    for y in 0..width {
        let char = board[x][y];
        if !root.chars.contains_key(&char){
            continue
        }
        dfs(&mut board, root.chars.get_mut(&char).unwrap(), &words, &mut res, x, y, &dirs, width, height);
    }
}

return res;
}

#[cfg(test)]
mod test {
    use super::find_words;

    #[test]
    fn base_case() {
        let matrix = [
            ["o", "a", "a", "n"],
            ["e", "t", "a", "e"],
            ["i", "h", "k", "r"],
            ["i", "f", "l", "v"],
        ];
        let words = ["oath", "pea", "eat", "rain"];
        assert_eq!(
            find_words(
                matrix
                    .iter()
                    .map(|row| row
                        .iter()
                        .map(|char| char.chars().nth(0).unwrap())
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                words
                    .iter()
                    .map(|word| word.to_string())
                    .collect::<Vec<_>>()
            ),
            ["eat", "oath"]
        );
    }
    #[test]
    fn single_letter() {
        let matrix = [["a"]];
        let words = ["a"];
        assert_eq!(
            find_words(
                matrix
                    .iter()
                    .map(|row| row
                        .iter()
                        .map(|char| char.chars().nth(0).unwrap())
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                words
                    .iter()
                    .map(|word| word.to_string())
                    .collect::<Vec<_>>()
            ),
            ["a"]
        );
    }
    #[test]
    fn multipple_match_case() {
        let matrix = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]];
        let words =["oath","pea","eat","rain","hklf", "hf"];
        assert_eq!(
            find_words(
                matrix
                    .iter()
                    .map(|row| row
                        .iter()
                        .map(|char| char.chars().nth(0).unwrap())
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                words
                    .iter()
                    .map(|word| word.to_string())
                    .collect::<Vec<_>>()
            ),
            ["oath","eat","hklf","hf"]
        );
    }
}
