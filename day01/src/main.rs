use std::collections::BinaryHeap;
use std::cmp::Reverse;

// find result in a *single* pass:
//   go line by line.  Detect Elf changes.  Track the one with the biggest sum.
fn first_half(lines: &[&str]) {
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

fn second_half(lines: &[&str]) {

    let mut heap = BinaryHeap::new();
    let mut current_sum = 0;

    // make array with sums
    for line in lines {
        if line.is_empty() {
            heap.push(Reverse(current_sum));
            if heap.len() > 3 {
                heap.pop();
            }
            current_sum = 0;
        } else {
            current_sum += str::parse::<i32>(line).unwrap();
        }
    }

    let res: i32 = heap.iter().map(|Reverse(val)| val).sum();

    println!("2) Sum of top three: {}", res);
}

fn main() -> std::io::Result<()> {
    let content = utils::load_data()?;
    let lines: Vec<&str> = content.lines().collect();

    first_half(&lines);
    second_half(&lines);

    Ok(())
}
