use utils;

#[derive(PartialEq, Eq, Debug)]
struct Range {
    from: u8,
    to: u8
}

impl Range {

    fn new(text: &str) -> Self {

        // no error checks
        let (from, to) = text.split_once("-").unwrap();
        return Self { from: from.parse::<u8>().unwrap(), to: to.parse::<u8>().unwrap() };
    }

    fn contains(&self, other: &Range) -> bool {
        return self.from <= other.from && self.to >= other.to; 
    }

    fn either_contains(&self, other: &Range) -> bool {
        return self.contains(other) || other.contains(self) 
    }
}

/// convert string like "83-83,57-89" to pair of Ranges
fn to_pair(line: &str) -> (Range, Range) {

    let (a,b) = line.split_once(",").unwrap();
    return (Range::new(a), Range::new(b));

}

fn main() -> std::io::Result<()> {
    
    let content = utils::load_data()?;
    let pairs = content.lines().map(to_pair);
    let count1 = pairs
                        .filter(|(a,b)|a.either_contains(b))
                        .count();
    println!("1) The count is {}", count1);


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_contained() {
        assert!(Range { from: 0, to: 3}.contains(&Range { from: 1, to: 2 }));
        assert!(Range { from: 0, to: 3}.contains(&Range { from: 1, to: 3 }));
        assert!(Range { from: 0, to: 3}.contains(&Range { from: 0, to: 2 }));
        assert!(! Range { from: 0, to: 3}.contains(&Range { from: 1, to: 5 }));
    }

    #[test]
    fn pair_making() {
        assert_eq!(to_pair("83-83,57-89"), (Range{from: 83, to: 83}, Range{from: 57, to: 89}));
    }
}