pub fn is_equal(str1: &[u8], str2: &[u8]) -> bool {
    let mut swapped_amount = 0;
    for index in 0..str1.len() {
        if str1[index] == str2[index] {
            continue;
        }
        if swapped_amount == 2 {
            return false;
        }
        swapped_amount += 1;
    }
    return true;
}
fn union(parent: usize, child: usize, graph: &mut Vec<usize>) -> bool {
    let parent_p = find(parent, graph);
    let child_p = find(child, graph);
    if parent_p == child_p {
        return false;
    }
    graph[child_p] = parent_p;
    return true;
}
fn find(node: usize, graph: &mut Vec<usize>) -> usize {
    let mut p = node;
    while p != graph[p] {
        let compressed = graph[graph[p]];
        graph[p] = compressed;
        p = compressed;
    }
    return p;
}
pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let mut graph: Vec<usize> = Vec::with_capacity(strs.len());
    let string_bytes: Vec<_> = strs.iter().map(|char| char.as_bytes()).collect();
    let mut amount = strs.len();
    for index in 0..strs.len() {
        graph.push(index);
    }

    for index in 0..strs.len() {
        for index2 in index + 1..strs.len() {
            let is_eq = is_equal(string_bytes[index], string_bytes[index2]);
            if !is_eq {
                continue;
            }
            let is_unied = union(index, index2, &mut graph);
            if !is_unied {
                continue;
            }
            amount -= 1;
        }
    }

    return amount as i32;
}

#[cfg(test)]
mod test {
    use crate::similar_string_groups::is_equal;

    use super::num_similar_groups;

    #[test]
    fn base_test() {
        let sample_data = ["tars", "rats", "arts", "star"];
        assert_eq!(
            num_similar_groups(sample_data.iter().map(|str| str.to_string()).collect()),
            2
        )
    }
    #[test]
    fn repeat_edge_case() {
        let sample_data = [
            "kccomwcgcs",
            "socgcmcwkc",
            "sgckwcmcoc",
            "coswcmcgkc",
            "cowkccmsgc",
            "cosgmccwkc",
            "sgmkwcccoc",
            "coswmccgkc",
            "kowcccmsgc",
            "kgcomwcccs",
        ];
        assert_eq!(
            num_similar_groups(sample_data.iter().map(|str| str.to_string()).collect()),
            5
        )
    }
    #[test]
    fn equality_test() {
        assert_eq!(is_equal("tars".as_bytes(), "rats".as_bytes()), true);
        assert_eq!(is_equal("tars".as_bytes(), "star".as_bytes()), false);
        assert_eq!(is_equal("rats".as_bytes(), "star".as_bytes()), false);
    }
}
