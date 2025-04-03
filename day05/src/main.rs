use utils;

type Strings<'a> = Vec<&'a str>;
type Stack = Vec<char>;

fn prepare_data(content: &String) {

    let mut stacks_data: Strings = Vec::new();
    let mut moves_data: Strings = Vec::new();
    let parts: [&mut Strings; 2] = [&mut stacks_data, &mut moves_data];

    let mut index: usize = 0; // initially put lines into `stacks_data`
    for line in content.lines() {

        if line.is_empty() {
            assert_ne!(index, 1);
            index = 1; // now put it into `moves_data`
            continue;
        }

        parts[index].push(line);
    }

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
        let curr_stack = &mut stacks[i];

        for line in stacks_data.iter() {
            let crate_name = line.as_bytes()[letter_index] as char;
            if crate_name == ' ' { break; } // stack ended

            curr_stack.push(crate_name);
        }
    }

}

fn main() -> std::io::Result<()> {

    let content = utils::load_data()?;
    prepare_data(&content);

    Ok(())
}
