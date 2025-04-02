struct Range {
    from: u8,
    to: u8
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return self.from <= other.from && self.to >= other.to; 
    }

    fn either_contains(&self, other: &Range) -> bool {
        return self.contains(other) || other.contains(self)
    }
}



fn main() {
    println!("Hello, world!");
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
}