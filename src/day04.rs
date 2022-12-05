struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn from_str(s: &str) -> Option<Self> {
        let mut iter = s.split('-');
        let from = iter.next()?.parse::<u64>().ok()?;
        let to = iter.next()?.parse::<u64>().ok()?;
        Some(Self { from, to })
    }

    fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlap(&self, other: &Self) -> bool {
        if self.from < other.from {
            self.to >= other.from
        } else {
            other.to >= self.from
        }
    }
}

pub struct CampCleanup {
    pairs: Vec<(Range, Range)>,
}

impl crate::AdventOfCode for CampCleanup {
    const TITLE: &'static str = "Camp Cleanup";
    const DAY: u8 = 4;

    fn new(input: &str) -> Option<Self> {
        let mut pairs = Vec::new();

        for line in input.lines() {
            let mut iter = line.split(",");
            let a = Range::from_str(iter.next()?)?;
            let b = Range::from_str(iter.next()?)?;
            pairs.push((a, b));
        }

        Some(Self { pairs })
    }

    fn part1(&self) -> u64 {
        self.pairs
            .iter()
            .filter(|(a, b)| a.contains(b) || b.contains(a))
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.pairs.iter().filter(|(a, b)| a.overlap(b)).count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn part1() {
        let res = CampCleanup::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 2);
    }

    #[test]
    fn part2() {
        let res = CampCleanup::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 4);
    }
}
