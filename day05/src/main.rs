use utils;

type Strings<'a> = Vec<&'a str>;
type Stack = Vec<char>;

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

fn prepare_data(content: &String) {

    let (mut stacks_data, _moves_data) = split_by_empty(content);

    let _stacks = extract_stacks(&mut stacks_data); // TODO: avoid mutability
    // // get number of stacks from string like: ` 1   2   3 `
    // // three chars per column + one space char between columns,
    // // so it's four chars per column
    // // the last column doesn't have a separator, so "emulate" it
    // let line_with_numbers = stacks_data.pop().unwrap();
    // let stacks_count = (line_with_numbers.len() + 1) / 4;

    // stacks_data.reverse();

    // let mut stacks: Vec<Stack> = Vec::with_capacity(stacks_count);

    // for i in 0..stacks_count {

    //     // get letter from string like `[Z] [M] [P]`
    //     let letter_index = i * 4 + 1;
    //     let curr_stack = &mut stacks[i];

    //     for line in stacks_data.iter() {
    //         let crate_name = line.as_bytes()[letter_index] as char;
    //         if crate_name == ' ' { break; } // stack ended

    //         curr_stack.push(crate_name);
    //     }
    // }

}

fn main() -> std::io::Result<()> {

    let content = utils::load_data()?;
    prepare_data(&content);

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
}