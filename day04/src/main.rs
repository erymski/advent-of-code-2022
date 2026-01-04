#[derive(PartialEq, Eq, Debug)]
struct Range {
    from: u8,
    to: u8,
}

impl Range {
    fn new(text: &str) -> Self {
        // no error checks
        let (from, to) = text.split_once("-").unwrap();
        Self {
            from: from.parse::<u8>().unwrap(),
            to: to.parse::<u8>().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn either_contains(&self, other: &Range) -> bool {
        self.contains(other) || other.contains(self)
    }

    fn overlap(&self, other: &Range) -> bool {
        self.any_inside(other) || other.any_inside(self)
    }

    fn inside(&self, pt: u8) -> bool {
        (self.from <= pt) && (pt <= self.to)
    }

    fn any_inside(&self, other: &Range) -> bool {
        self.inside(other.from) || self.inside(other.to)
    }
}

/// convert string like "83-83,57-89" to pair of Ranges
fn to_pair(line: &str) -> (Range, Range) {
    let (a, b) = line.split_once(",").unwrap();
    (Range::new(a), Range::new(b))
}

fn main() -> std::io::Result<()> {
    let content = utils::load_data()?;
    let pairs: Vec<(Range, Range)> = content.lines().map(to_pair).collect(); // TODO: how to convert to array?

    let count1 = pairs.iter().filter(|(a, b)| a.either_contains(b)).count();
    println!("1) The count is {}", count1);

    let count2 = pairs.iter().filter(|(a, b)| a.overlap(b)).count();
    println!("2) The count is {}", count2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_contained() {
        assert!(Range { from: 0, to: 3 }.contains(&Range { from: 1, to: 2 }));
        assert!(Range { from: 0, to: 3 }.contains(&Range { from: 1, to: 3 }));
        assert!(Range { from: 0, to: 3 }.contains(&Range { from: 0, to: 2 }));
        assert!(!Range { from: 0, to: 3 }.contains(&Range { from: 1, to: 5 }));
    }

    #[test]
    fn inside_check() {
        assert!(Range { from: 0, to: 3 }.inside(1));
        assert!(Range { from: 0, to: 3 }.inside(0));
        assert!(Range { from: 0, to: 3 }.inside(3));
        assert!(!Range { from: 1, to: 3 }.inside(0));
        assert!(!Range { from: 0, to: 3 }.inside(4));
    }

    #[test]
    fn overlapping() {
        assert!(Range { from: 0, to: 3 }.overlap(&Range { from: 1, to: 2 }));
        assert!(Range { from: 0, to: 3 }.overlap(&Range { from: 1, to: 3 }));
        assert!(Range { from: 0, to: 3 }.overlap(&Range { from: 0, to: 2 }));
        assert!(Range { from: 0, to: 3 }.overlap(&Range { from: 1, to: 5 }));
        assert!(Range { from: 0, to: 3 }.overlap(&Range { from: 3, to: 5 }));
        assert!(!Range { from: 0, to: 3 }.overlap(&Range { from: 4, to: 5 }));
    }

    #[test]
    fn pair_making() {
        assert_eq!(
            to_pair("83-83,57-89"),
            (Range { from: 83, to: 83 }, Range { from: 57, to: 89 })
        );
    }
}
