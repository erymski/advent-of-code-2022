use std::collections::BinaryHeap;
use std::cmp::Reverse;

const SEPARATOR: i32 = -1;

// find result in a *single* pass:
//   go line by line.  Detect Elf changes.  Track the one with the biggest sum.
fn first_half(series: &[i32]) -> (i32, i32) {

    let mut curr_sum: i32 = 0;
    let mut curr_index: i32 = 0;
    let mut biggest_index: i32 = -1;
    let mut biggest_sum: i32 = -1;

    let mut process = |sum: i32, index: i32| {
        if sum > biggest_sum {
            biggest_index = index;
            biggest_sum = sum;
        }
    };

    for &num in series {
        if num == SEPARATOR {

            process(curr_sum, curr_index);

            curr_sum = 0;
            curr_index += 1;

        } else {
            curr_sum += num;
        }
    }

    process(curr_sum, curr_index);

    return (biggest_index, biggest_sum);
}

fn second_half(series: &[i32]) -> i32 {

    let mut heap = BinaryHeap::new();

    let mut update_heap = |value: i32| {
        heap.push(Reverse(value));
        if heap.len() > 3 {
            heap.pop();
        }
    };

    let mut current_sum = 0;
    for &num in series {
        if num == SEPARATOR {
            update_heap(current_sum);
            current_sum = 0;
        } else {
            current_sum += num;
        }
    }

    update_heap(current_sum);

    let res: i32 = heap.iter().map(|Reverse(val)| val).sum();
    return res;
}

fn main() -> std::io::Result<()> {
    let content = utils::load_data()?;
    let series: Vec<i32> = content.lines().map(|l| if l.is_empty() { SEPARATOR } else { l.parse().unwrap() }).collect();

    let (biggest_index, biggest_sum) = first_half(&series);
    println!("1) Found: index: {}, sum: {}", biggest_index, biggest_sum);

    let res = second_half(&series);
    println!("2) Sum of top three: {}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half_basic() {
        let series = vec![100, 200, SEPARATOR, 300, SEPARATOR, 50, 150];
        let (index, sum) = first_half(&series);
        assert_eq!(index, 0);
        assert_eq!(sum, 300);
    }

    #[test]
    fn test_first_half_single_elf() {
        let series = vec![100, 200];
        let (index, sum) = first_half(&series);
        assert_eq!(index, 0);
        assert_eq!(sum, 300);
    }

    #[test]
    fn test_first_half_last_is_max() {
        let series = vec![100, SEPARATOR, 200, 300];
        let (index, sum) = first_half(&series);
        assert_eq!(index, 1);
        assert_eq!(sum, 500);
    }

    #[test]
    fn test_second_half_basic() {
        let series = vec![100, 200, SEPARATOR, 300, SEPARATOR, 50, 150, SEPARATOR, 400];
        let sum = second_half(&series);
        assert_eq!(sum, 1000); // 400 + 300 + 300
    }


    #[test]
    fn test_second_half_single_large() {
        let series = vec![1000, SEPARATOR, 100, SEPARATOR, 200];
        let sum = second_half(&series);
        assert_eq!(sum, 1300); // 1000 + 200 + 100
    }
}