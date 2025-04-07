use utils;

type Strings<'a> = Vec<&'a str>;
type Stack = Vec<char>;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_REGEX: Regex = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
}

struct Move {
    count: u8,
    from: u8,
    to: u8
}

impl Move {
    fn parse(line: &str) -> Self { // TODO: parse without regex
        
        if let Some(captures) = MOVE_REGEX.captures(line) {

            debug_assert_eq!(captures.len(), 4);

            return Self { 
                count: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(), // TODO: yuck
                from: captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                to: captures.get(3).unwrap().as_str().parse::<u8>().unwrap() }

        } else {
            panic!("Unexpected move format");
        }
    }
}

fn split_by_empty(content: &String) -> (Strings, Strings) {

    let mut first: Strings = Vec::new();
    let mut second: Strings = Vec::new();
    let parts: [&mut Strings; 2] = [&mut first, &mut second];

    let mut index: usize = 0; // initially put lines into first vector
    for line in content.lines() {

        if line.is_empty() {
            assert_ne!(index, 1, "Not expecting more than one empty lines");
            index = 1; // now put it into second vector
            continue;
        }

        parts[index].push(line);
    }

    return (first, second)
}

fn extract_stacks(stacks_data: &mut Strings) -> Vec<Stack> {
    // get number of stacks from string like: ` 1   2   3 `
    // three chars per column + one space char between columns,
    // so it's four chars per column
    // the last column doesn't have a separator, so "emulate" it
    let line_with_numbers = stacks_data.pop().unwrap();
    let stacks_count = (line_with_numbers.len() + 1) / 4;

    stacks_data.reverse();

    let mut stacks: Vec<Stack> = Vec::with_capacity(stacks_count);

    for i in 0..stacks_count {

        // get letter from string like `[Z] [M] [P]`
        let letter_index = i * 4 + 1;
        stacks.push(Vec::new());
        let curr_stack = &mut stacks[i];

        for line in stacks_data.iter() {
            let crate_name = line.as_bytes()[letter_index] as char;
            if crate_name == ' ' { break; } // stack ended

            curr_stack.push(crate_name);
        }
    }

    return stacks;
}

fn extract_moves(moves_data: &Strings) -> Vec<Move> { // TODO: do with CUDA?

    // string like `move 1 from 2 to 1`
    return moves_data.iter()
            .map(|line| Move::parse(line))
            .collect()
}

fn prepare_data(content: &String) -> (Vec<Stack>, Vec<Move>) {

    let (mut stacks_data, moves_data) = split_by_empty(content);

    let stacks = extract_stacks(&mut stacks_data); // TODO: avoid mutability of input
    let moves = extract_moves(& moves_data);

    return (stacks, moves);
}

fn part_1(content: &String) -> String {

    let (mut stacks, moves) = prepare_data(&content);

    for m in moves {

        // TODO: now addressing required stack is done in the loop, because:
        // - cannot borrow two stacks at once
        // - tricky way to have immutable vector of mutable vectors

        // let from_stack = &mut stacks[(m.from - 1) as usize];
        // let to_stack = &mut stacks[(m.to - 1) as usize];

        for _i in 0..m.count {
            let top = stacks[(m.from - 1) as usize].pop();
            if top.is_some() {
                stacks[(m.to - 1) as usize].push(top.unwrap());
            }
            // to_stack.push(from_stack.pop().unwrap());
        }
    }

    let mut result = String::new();
    for mut s in stacks {
        let top = s.pop();
        if top.is_some() {
            result.push(top.unwrap());
        }
    }

    return result;
}

fn main() -> std::io::Result<()> {

    let content = utils::load_data()?;
    let result1 = part_1(&content);
    println!("1) Tops are {}", result1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split() {
        let input: String = String::from("a\nb\n\nc\nd");
        let (first, second) = split_by_empty(&input);
        assert_eq!(first.len(), 2);
        assert_eq!(second.len(), 2);
        assert_eq!(first[0], "a");
        assert_eq!(first[1], "b");
        assert_eq!(second[0], "c");
        assert_eq!(second[1], "d");
    }

    #[test]
    fn stacks() {
        let mut input = vec![
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 "
        ];

        let stacks = extract_stacks(&mut input);
        assert_eq!(stacks.len(), 3);

        assert_eq!(stacks[0], Vec::from(['Z', 'N']));
        assert_eq!(stacks[1], Vec::from(['M', 'C', 'D']));
        assert_eq!(stacks[2], Vec::from(['P']));
    }

    #[test]
    fn move_parse() {
        let m = Move::parse("move 19 from 2 to 1");
        assert_eq!(m.count, 19);
        assert_eq!(m.from, 2);
        assert_eq!(m.to, 1);
    }
}