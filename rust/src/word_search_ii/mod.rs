#[derive(Debug, Default)]
struct Node {
    chars: [Children; 26],
    amount: i32,
    end: Option<usize>,
}
type Children = Option<Box<Node>>;
impl Node {
    fn contains_key(&self, key: char) -> bool {
        let i = key as usize - 'a' as usize;
        return self.chars[i].is_some();
    }
    fn get(&self, key: char) -> &Children {
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
}
fn dfs(
    board: &mut Vec<Vec<char>>,
    node: &mut Box<Node>,
    words: &Vec<String>,
    res: &mut Vec<String>,
    x: usize,
    y: usize,
    dirs: &[i32],
    width: usize,
    height: usize,
)->i32 {
    let mut founded = 0;
    if let Some(index) = node.end {
        founded+=1;
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
        if next == '1'{
            continue;
        }
        
        if node.contains_key(next) {
            let p = node.get_mut(next).as_mut().unwrap();
            founded += dfs(
                board,
                p,
                words,
                res,
                dx,
                dy,
                dirs,
                width,
                height,
            );
            if p.amount == 0{
                node.delete(next);
            }
        }
    }
    node.amount-=founded;
    board[x][y] = temp;
    return founded
}

pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(words.len());
    let mut root: Children = None;
    for (index, word) in words.iter().enumerate() {
        let mut cur = &mut root;
        for char in word.chars() {
            let next_node = cur.get_or_insert_with(Box::default);
            next_node.amount += 1;
            cur = next_node.get_mut(char);
        }
        let last = cur.get_or_insert_with(Box::default);
        last.amount += 1;
        last.end = Some(index);
    }
    let Some(root) = &mut root else {
        return vec![];
    };
    let width = board[0].len();
    let height = board.len();
    let dirs = [0, 1, 0, -1, 0];
    'outer: for x in 0..height {
        for y in 0..width {
            let char = board[x][y];
            if !root.contains_key(char) {
                continue;
            }
            let founded = dfs(
                &mut board,
                root.get_mut(char).as_mut().unwrap(),
                &words,
                &mut res,
                x,
                y,
                &dirs,
                width,
                height,
            );
            root.amount -= founded;
            if root.amount == 0{
                break 'outer;
            }
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
        let matrix = [
            ["o", "a", "a", "n"],
            ["e", "t", "a", "e"],
            ["i", "h", "k", "r"],
            ["i", "f", "l", "v"],
        ];
        let words = ["oath", "pea", "eat", "rain", "hklf", "hf"];
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
            ["oath", "eat", "hklf", "hf"]
        );
    }
}
