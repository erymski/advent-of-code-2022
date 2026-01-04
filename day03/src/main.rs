use std::collections::HashMap;
use std::collections::HashSet;

/// Find common char in both halves of the string
fn find_shared(line: &str) -> Option<char> {
    let len: usize = line.len();
    if len == 0 {
        return None;
    }

    let (first, second) = line.split_at(len / 2);

    let first_half_chars: HashSet<char> = first.chars().collect();

    second.chars().find(|&ch| first_half_chars.contains(&ch))
}

/// Convert character to corresponding priority
fn to_number(ch: char) -> u8 {
    const A_LOW: u8 = b'a';
    const A_UPPER: u8 = b'A';

    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let as_byte = ch as u8;
    if as_byte >= A_LOW {
        as_byte - A_LOW + 1
    } else {
        as_byte - A_UPPER + 27
    }
}

fn get_badge_letter(lines: &[&str]) -> Option<char> {
    // TODO: can it be iterator over strings?

    let mut chars_count: HashMap<char, u8> = HashMap::new();

    let mut bit = 1u8;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        for ch in trimmed.chars() {
            let stat = chars_count.entry(ch).or_insert(0);
            *stat |= bit;
        }

        bit <<= 1;
    }

    for (ch, bits) in chars_count {
        if bits == 0b111 {
            return Some(ch);
        } // expecting three lines
    }

    None
}

/// Solve first part of the day 3
fn first_part(lines: &[&str]) -> u32 {

    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| find_shared(line).unwrap())
        .map(|shared| to_number(shared) as u32)
        .sum()
}

fn second_part(lines: &Vec<&str>) -> u32 {

    lines
        .chunks(3)
        .map(|triple| get_badge_letter(triple).unwrap())
        .map(|letter| to_number(letter) as u32)
        .sum()
}

fn main() -> std::io::Result<()> {
    let content = utils::load_data()?;
    let lines: Vec<&str> = content.lines().collect();

    let sum1: u32 = first_part(&lines);
    println!("1) The sum is {}", sum1);

    let sum2: u32 = second_part(&lines);
    println!("2) The sum is {}", sum2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_shared_success() {
        assert_eq!(find_shared("vJrwpWtwJgWrhcsFMMfFFhFp"), Some('p'));
        assert_eq!(find_shared("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), Some('L'));
        assert_eq!(find_shared("PmmdzqPrVvPwwTWBwg"), Some('P'));
        assert_eq!(find_shared("ttgJtRGJQctTZtZT"), Some('t'));
        assert_eq!(find_shared("CrZsJsPPZsGzwwsLwLmpwMDw"), Some('s'));
    }

    #[test]
    fn find_shared_failure() {
        assert_eq!(find_shared("line"), None);
        assert_eq!(find_shared(""), None);
    }

    #[test]
    fn to_number_conversion() {
        assert_eq!(to_number('p'), 16);
        assert_eq!(to_number('L'), 38);
        assert_eq!(to_number('P'), 42);
        assert_eq!(to_number('v'), 22);
        assert_eq!(to_number('t'), 20);
        assert_eq!(to_number('s'), 19);
    }

    #[test]
    fn get_badge() {
        let input1 = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        assert_eq!(get_badge_letter(&input1), Some('r'));

        let input2 = [
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(get_badge_letter(&input2), Some('Z'));
    }
}
