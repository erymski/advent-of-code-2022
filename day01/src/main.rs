use std::fs;
use std::env;

// find result in a *single* pass:
//   go line by line.  Detect Elf changes.  Track the one with the biggest sum.
fn first_half(lines: &Vec<&str>) {

    let mut curr_sum: i32 = 0;
    let mut curr_index: i32 = 0;
    let mut biggest_index: i32 = -1;
    let mut biggest_sum: i32 = -1;

    for line in lines {

        if line.is_empty() {

            if curr_sum > biggest_sum {
                biggest_index = curr_index;
                biggest_sum = curr_sum;
            }

            curr_sum = 0;
            curr_index += 1;
        } else {

            curr_sum += str::parse::<i32>(line).unwrap();
        }
    }

    println!("1) Found: index: {}, sum: {}", biggest_index, biggest_sum);

}

fn second_half(lines: &Vec<&str>) {

    let mut vec: Vec<i32> = vec![0];
    let mut index: usize = 0;

    // make array with sums
    for line in lines {

        if line.is_empty() {
            index += 1;
            vec.push(0);
        }
        else {
            vec[index] += str::parse::<i32>(line).unwrap();
        }
    }

    // sort the resulting array and sum up the last three items
    vec.sort();
    let last3 = &vec[vec.len()-3..];
    let res: i32 = last3.iter().sum();

    println!("2) Sum of top three: {}", res);
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Not enough command line arguments"); }

    let filename: &String = &args[1];
    println!("\nIncoming path: {}", filename);

    let content = fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect();

    first_half(&lines);
    second_half(&lines);

    Ok(())
}
