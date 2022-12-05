pub struct RockPaperScissors {
    instructions: Vec<(char, char)>,
}

impl crate::AdventOfCode for RockPaperScissors {
    const TITLE: &'static str = "Rock Paper Scissors";
    const DAY: u8 = 2;

    fn new(input: &str) -> Option<Self> {
        let mut instructions = Vec::new();

        for line in input.lines() {
            let mut iter = line.chars();
            let first_column = iter.next()?;
            let second_column = iter.nth(1)?;
            instructions.push((first_column, second_column));
        }

        Some(RockPaperScissors { instructions })
    }

    fn part1(&self) -> u64 {
        let mut score = 0;

        for instruction in &self.instructions {
            score += match instruction.1 {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            };

            score += match instruction {
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => 3,
            };
        }

        score
    }

    fn part2(&self) -> u64 {
        let mut score = 0;

        for instruction in &self.instructions {
            score += match instruction.1 {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => 0,
            };

            score += match instruction {
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
                ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
                _ => 0,
            };
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1() {
        let res = RockPaperScissors::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 15);
    }

    #[test]
    fn part2() {
        let res = RockPaperScissors::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 12);
    }
}
