use std::collections::BinaryHeap;
use std::cmp::Reverse;

const SEPARATOR: i32 = -1;

// find result in a *single* pass:
//   go line by line.  Detect Elf changes.  Track the one with the biggest sum.
fn first_half(series: &[i32]) -> (usize, i32) {

    let sums = series
        .split(|&x| x == SEPARATOR)
        .map(|group| group.iter().sum::<i32>());

    let mut biggest_index: usize = 0;
    let mut biggest_sum: i32 = -1;

    for (index, sum) in sums.enumerate() {
        if sum > biggest_sum {
            biggest_index = index;
            biggest_sum = sum;
        }
    }

    (biggest_index, biggest_sum)
}

fn second_half(series: &[i32]) -> i32 {

    // group and sum
    let sums = series
            .split(|&x| x == SEPARATOR)
            .map(|group| group.iter().sum::<i32>());

    // maintain a min-heap of top 3
    let mut heap = BinaryHeap::new();
    for sum in sums {
        heap.push(Reverse(sum));
        if heap.len() > 3 {
            heap.pop();
        }
    }

    // get sum of the top three items
    heap.iter().map(|Reverse(val)| val).sum()
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
        assert_eq!(index, 0, "The index of the elf with the maximum sum should be 0, not {0}", index);
        assert_eq!(sum, 300, "The maximum sum should be 300, not {0}", sum);
    }

    #[test]
    fn test_first_half_single_elf() {
        let series = vec![100, 200];
        let (index, sum) = first_half(&series);
        assert_eq!(index, 0, "With a single elf, the index should be 0");
        assert_eq!(sum, 300, "The sum for the single elf should be 300");
    }

    #[test]
    fn test_first_half_last_is_max() {
        let series = vec![100, SEPARATOR, 200, 300];
        let (index, sum) = first_half(&series);
        assert_eq!(index, 1, "The last elf should have the maximum sum, so index should be 1");
        assert_eq!(sum, 500, "The maximum sum should be 500");
    }

    #[test]
    fn test_second_half_basic() {
        let series = vec![100, 200, SEPARATOR, 300, SEPARATOR, 50, 150, SEPARATOR, 400];
        let sum = second_half(&series);
        assert_eq!(sum, 1000, "The sum of the top three elf sums should be 1000");
    }

    #[test]
    fn test_second_half_single_large() {
        let series = vec![1000, SEPARATOR, 100, SEPARATOR, 200];
        let sum = second_half(&series);
        assert_eq!(sum, 1300, "The sum of the top three elf sums should be 1300");
    }
}