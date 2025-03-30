use std::fs;
use std::env;
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

const A_LOW: u8 = 'a' as u8;
const A_UPPER: u8 = 'A' as u8;

fn to_number(ch: char) -> u8 {
    let as_byte = ch as u8;
    if as_byte >= A_LOW {as_byte - A_LOW + 1} else { as_byte - A_UPPER + 27 }
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Not enough command line arguments"); }

    let filename: &String = &args[1];
    println!("\nIncoming path: {}", filename);

    let content = fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect();

    let mut sum: u32 = 0;
    for line in &lines {

        if line.is_empty() {

        } else {
            let shared = find_shared(line);
            if shared.is_some() {
                sum += to_number(shared.unwrap()) as u32
            }
        }
    }

    println!("1) The sum is {}", sum);


    Ok(())
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

    #[test]
    fn test_to_char() {
        assert_eq!(to_number('p'), 16);
        assert_eq!(to_number('L'), 38);
        assert_eq!(to_number('P'), 42);
        assert_eq!(to_number('v'), 22);
        assert_eq!(to_number('t'), 20);
        assert_eq!(to_number('s'), 19);
    }
}
