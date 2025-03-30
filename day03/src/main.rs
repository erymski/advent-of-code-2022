use std::collections::HashSet;

fn find_shared(line: &str) -> Option<char> {

    let len: usize = line.len();
    if len == 0 { return None; }

    let (first, second) = line.split_at(len / 2);

    let mut first_half_chars: HashSet<char> = HashSet::new();
    for ch in first.chars() {
        first_half_chars.insert(ch);
    }

    for ch in second.chars() {
        if first_half_chars.contains(&ch) { return Some(ch); }
    }

    return None;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shared_success() {
        assert_eq!(find_shared("vJrwpWtwJgWrhcsFMMfFFhFp"), Some('p'));
        assert_eq!(find_shared("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), Some('L'));
        assert_eq!(find_shared("PmmdzqPrVvPwwTWBwg"), Some('P'));
        assert_eq!(find_shared("ttgJtRGJQctTZtZT"), Some('t'));
        assert_eq!(find_shared("CrZsJsPPZsGzwwsLwLmpwMDw"), Some('s'));
    }

    #[test]
    fn test_find_shared_failure() {
        assert_eq!(find_shared("line"), None);
        assert_eq!(find_shared(""), None);
    }
}
