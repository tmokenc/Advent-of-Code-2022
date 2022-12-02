use std::mem;

pub struct CalorieCounting {
    bags: Vec<Vec<u64>>,
}

impl crate::AdventOfCode for CalorieCounting {
    const TITLE: &'static str = "Calorie Counting";
    const DAY: u8 = 1;

    fn new(input: &str) -> Option<Self> {
        let mut bags = Vec::new();
        let mut bag = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                bags.push(mem::take(&mut bag));
                continue;
            }

            let calorie = line.parse::<u64>().ok()?;
            bag.push(calorie);
        }

        bags.push(bag);

        Some(CalorieCounting { bags })
    }

    fn part1(&self) -> u64 {
        self.bags.iter().map(|v| v.iter().sum()).max().unwrap_or_default()
    }

    fn part2(&self) -> u64 {
        let mut top3 = [0; 3];

        for calories in self.bags.iter().map(|v| v.iter().sum()) {
            if calories > top3[0] {
                top3[0] = calories;
                top3.sort_unstable();
            }
        }

        top3.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part1() {
        let res = CalorieCounting::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 24000);
    }

    #[test]
    fn part2() {
        let res = CalorieCounting::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 45000);
    }
}
