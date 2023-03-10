use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Seek};

// find result in a *single* pass:
//   go line by line.  Detect Elf changes.  Track the one with the biggest sum.
fn first_half(file: &File) -> std::io::Result<()> {

    let reader = BufReader::new(file);
    
    let mut curr_sum: i32 = 0;
    let mut curr_index: i32 = 0;
    let mut biggest_index: i32 = -1;
    let mut biggest_sum: i32 = -1;


    for line in reader.lines() {

        let line = line?;
        if line.is_empty() {

            if curr_sum > biggest_sum {
                biggest_index = curr_index;
                biggest_sum = curr_sum;
            }

            curr_sum = 0;
            curr_index += 1;
        } else {

            curr_sum += str::parse::<i32>(line.as_str()).unwrap();
        }
    }

    println!("1) Found: index: {}, sum: {}", biggest_index, biggest_sum);

    Ok(())

}

fn second_half(file: &File) -> std::io::Result<()> {

    let mut vec = vec![0];
    let mut index = 0;

    let reader = BufReader::new(file);

    // make array with sums
    for line in reader.lines() {

        let line = line?;
        if line.is_empty() {
            index += 1;
            vec.push(0);
        }
        else {
            vec[index] += str::parse::<i32>(line.as_str()).unwrap();
        }
    }

    // sort the resulting array and sum up the last three items
    vec.sort();
    let last3 = &vec[vec.len()-3..];
    let res: i32 = last3.iter().sum();

    println!("2) Sum of top three: {}", res);

    Ok(())
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    println!("\nIncoming path: {}", filename);

    let mut file = File::open(filename)?;
    first_half(&file)?;

    file.rewind()?;
    second_half(&file)?;

    Ok(())
}

