fn fill_string_left(slected: &[String], res: &mut Vec<String>, words_len: usize, width: usize) {
    let mut result_str = String::new();
    let mut needed_space = (width - words_len) as i32;

    for (index, string) in slected.iter().enumerate() {
        result_str.push_str(string);
        if needed_space > 0 {
            result_str.push(' ');
            needed_space -= 1;
        }
    }
    if needed_space > 0 {
        for _ in 0..needed_space {
            result_str.push(' ');
        }
    }
    res.push(result_str);
}
fn fill_string_center(slected: &[String], res: &mut Vec<String>, words_len: usize, width: usize) {
    let mut result_str = String::new();
    let needed_space = (slected.len() as i32 - 1).max(1) as usize;
    let mut spaces_left = (width - words_len) as i32;

    let avg_space_lg = ((width - words_len) as f32 / needed_space as f32).ceil() as i32;
    let avg_space_sm = ((width - words_len) as f32 / needed_space as f32).floor() as i32;
    
    for (index, string) in slected.iter().enumerate() {
        let spaces_left_am = (needed_space as usize - index) as i32;
        let mut spaces_amount: i32 = avg_space_lg;
        if (spaces_left - avg_space_sm * spaces_left_am) == 0 {
            spaces_amount = avg_space_sm;
        }
        if index == 0 {
            result_str.push_str(string);
            for _ in 0..spaces_amount {
                result_str.push(' ');
            }
            spaces_left -= spaces_amount;
            continue;
        }
        result_str.push_str(string);
        if index < slected.len() - 1 {
            for _ in 0..spaces_amount {
                result_str.push(' ');
            }
            spaces_left -= spaces_amount;
        }
    }
    res.push(result_str);
}
pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut slection_index = 0;
    let max_width: usize = max_width as usize;
    while slection_index < words.len() {
        let mut word_len: usize = 0;
        let mut word_len_space: usize = 0;
        let mut inner_selection = slection_index;
        while inner_selection < words.len() {
            let len_w = words[inner_selection].len();

            if word_len_space + len_w + 1 <= max_width {
                word_len += len_w;
                word_len_space += len_w + 1;
                inner_selection += 1;
                continue;
            }
            if word_len_space + len_w <= max_width {
                word_len += len_w;
                inner_selection += 1;
            }
            break;
        }

        if inner_selection >= words.len() {
            fill_string_left(
                &words[slection_index..inner_selection],
                &mut res,
                word_len,
                max_width,
            );
            break;
        }
        fill_string_center(
            &words[slection_index..inner_selection],
            &mut res,
            word_len,
            max_width,
        );
        slection_index = inner_selection;
    }

    return res;
}

#[cfg(test)]
mod test {
    use crate::text_justification::full_justify;

    #[test]
    fn basic_justify_test() {
        let sample = [
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ];
        let assert_res = ["This    is    an", "example  of text", "justification.  "];

        assert_eq!(
            full_justify(sample.iter().map(|char| char.to_string()).collect(), 16),
            assert_res
        );
    }
    #[test]
    fn last_left_justify_test() {
        let sample = ["What", "must", "be", "acknowledgment", "shall", "be"];
        let assert_res = ["What   must   be", "acknowledgment  ", "shall be        "];

        assert_eq!(
            full_justify(sample.iter().map(|char| char.to_string()).collect(), 16),
            assert_res
        );
    }
    #[test]
    fn even_test_case() {
        let sample = [
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ];
        let assert_res = [
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ];

        assert_eq!(
            full_justify(sample.iter().map(|char| char.to_string()).collect(), 20),
            assert_res
        );
    }
    #[test]
    fn memory_limit_case() {
        let sample = [
            "ask", "not", "what", "your", "country", "can", "do", "for", "you", "ask", "what",
            "you", "can", "do", "for", "your", "country",
        ];
        let assert_res = [
            "ask   not   what",
            "your country can",
            "do  for  you ask",
            "what  you can do",
            "for your country",
        ];

        assert_eq!(
            full_justify(sample.iter().map(|char| char.to_string()).collect(), 16),
            assert_res
        );
    }
    #[test]
    fn large_edge_case() {
        let sample = ["Give","me","my","Romeo;","and,","when","he","shall","die,","Take","him","and","cut","him","out","in","little","stars,","And","he","will","make","the","face","of","heaven","so","fine","That","all","the","world","will","be","in","love","with","night","And","pay","no","worship","to","the","garish","sun."];
        let assert_res = ["Give  me  my  Romeo; and,","when  he  shall die, Take","him  and  cut  him out in","little stars, And he will","make  the  face of heaven","so   fine  That  all  the","world  will  be  in  love","with  night  And  pay  no","worship   to  the  garish","sun.                     "];

        assert_eq!(
            full_justify(sample.iter().map(|char| char.to_string()).collect(), 25),
            assert_res
        );
    }
}
