use utils;

type Strings<'a> = Vec<&'a str>;
type Stack = Vec<char>;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_REGEX: Regex = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
}

struct Move {
    count: usize,
    from: u8,
    to: u8
}

impl Move {
    fn parse(line: &str) -> Self { // TODO: parse without regex
        
        if let Some(captures) = MOVE_REGEX.captures(line) {

            debug_assert_eq!(captures.len(), 4);

            return Self { 
                count: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(), // TODO: yuck
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

fn extract_stacks(stacks_data: &Strings) -> Vec<Stack> {

    let len = stacks_data.len();
    let (stack_content, stack_indexes) = stacks_data.split_at(len - 1);
    debug_assert!(stack_indexes.len() == 1);

    // get number of stacks from string like: ` 1   2   3 `
    // three chars per column + one space char between columns,
    // so it's four chars per column
    // the last column doesn't have a separator, so "emulate" it
    let line_with_numbers = stack_indexes[0];
    let stacks_count = (line_with_numbers.len() + 1) / 4;

    let mut stacks: Vec<Stack> = Vec::with_capacity(stacks_count);

    for i in 0..stacks_count {

        // get letter from string like `[Z] [M] [P]`
        let letter_index = i * 4 + 1;
        stacks.push(Vec::new());
        let curr_stack = &mut stacks[i];

        for line in stack_content.iter().rev() {
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

    let (stacks_data, moves_data) = split_by_empty(content);

    let stacks = extract_stacks(&stacks_data);
    let moves = extract_moves(& moves_data);

    return (stacks, moves);
}

fn get_top_letters(stacks: &Vec<Stack>) -> String {
    let mut result = String::new();
    for s in stacks {

        if let Some(top) = s.last() {
            result.push(*top);
        }
    }

    return result;
}

type MoveFn = fn(&Move, &mut Vec<Stack>);

fn reverse_move(m: &Move, stacks: &mut Vec<Stack>) {

    // TODO: now addressing required stack is done in the loop, because:
    // - cannot borrow two stacks at once
    // - tricky way to have immutable vector of mutable vectors

    // let from_stack = &mut stacks[(m.from - 1) as usize];
    // let to_stack = &mut stacks[(m.to - 1) as usize];

    let from = (m.from - 1) as usize;
    let to = (m.to - 1) as usize;

    for _ in 0..m.count {
        if let Some(top) = stacks[from].pop() {
            stacks[to].push(top);
        }
    }
}

fn block_move(m: &Move, stacks: &mut Vec<Stack>) {

    // TODO: now addressing required stack is done in the loop, because:
    // - cannot borrow two stacks at once
    // - tricky way to have immutable vector of mutable vectors

    // let from_stack = &mut stacks[(m.from - 1) as usize];
    // let to_stack = &mut stacks[(m.to - 1) as usize];

    let from = (m.from - 1) as usize;
    let to = (m.to - 1) as usize;

    let len = stacks[from].len();
    debug_assert!(len >= m.count);
    let block = stacks[from].split_off(len - m.count);

    stacks[to].extend(block);
}

fn run_part(content: &String, move_operation: MoveFn) -> String {

    let (mut stacks, moves) = prepare_data(&content);

    for m in moves {

        move_operation(&m, &mut stacks);
    }

    return get_top_letters(&stacks);
}

fn main() -> std::io::Result<()> {

    let content = utils::load_data()?;

    let result1 = run_part(&content, reverse_move);
    println!("1) Tops are {}", result1);

    let result2 = run_part(&content, block_move);
    println!("2) Tops are {}", result2);

    Ok(())
}

lazy_static! {


}
    

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CRATES: [&str;4] = [
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 "
        ];

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
        let stacks = extract_stacks(&Vec::from(TEST_CRATES));
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

    #[test]
    fn top_letters() {

        let stacks = extract_stacks(&Vec::from(TEST_CRATES));

        let result = get_top_letters(&stacks);
        assert_eq!(result, "NDP");
    }

    #[test]
    fn check_reverse_move() {

        let mut stacks = extract_stacks(&Vec::from(TEST_CRATES));

        let m = Move { count: 2, from: 2, to: 3 };
        reverse_move(&m, &mut stacks);

        let result = get_top_letters(&stacks);
        assert_eq!(result, "NMC");
    }

    #[test]
    fn check_block_move() {
        let mut stacks = extract_stacks(&Vec::from(TEST_CRATES));

        let m = Move { count: 2, from: 2, to: 3 };
        block_move(&m, &mut stacks);

        let result = get_top_letters(&stacks);
        assert_eq!(result, "NMD");
    }

}