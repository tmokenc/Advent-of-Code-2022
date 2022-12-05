struct Instruction {
    take: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_str(s: &str) -> Option<Self> {
        let mut iter = s.split_whitespace();
        let mut res = Self {
            take: 0,
            from: 0,
            to: 0,
        };

        iter.next().filter(|&v| v == "move")?;
        res.take = iter.next()?.parse().ok()?;

        iter.next().filter(|&v| v == "from")?;
        res.from = iter.next()?.parse().ok()?;

        iter.next().filter(|&v| v == "to")?;
        res.to = iter.next()?.parse().ok()?;

        Some(res)
    }
}

#[derive(Default)]
pub struct SupplyStacks {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl crate::AdventOfCode<String> for SupplyStacks {
    const TITLE: &'static str = "Supply Stacks";
    const DAY: u8 = 5;

    fn new(input: &str) -> Option<Self> {
        let mut iter = input.lines();
        let mut res = Self::default();

        while let Some(line) = iter.next() {
            if line.starts_with(" 1 ") {
                iter.next();
                break;
            }

            let mut chars = line.chars();
            let mut current_idx = 0;

            while let Some(ch) = chars.next() {
                if current_idx == res.stacks.len() {
                    res.stacks.push(Default::default());
                }

                if ch == ' ' {
                    chars.next().filter(|&v| v == ' ')?;
                    chars.next().filter(|&v| v == ' ')?;
                } else if ch == '[' {
                    res.stacks[current_idx].insert(0, chars.next()?);
                    chars.next().filter(|&v| v == ']')?;
                } else {
                    return None;
                }

                chars.next();
                current_idx += 1;
            }
        }

        for line in iter {
            res.instructions.push(Instruction::from_str(line)?);
        }

        Some(res)
    }

    fn part1(&self) -> String {
        let mut stacks = self.stacks.clone();

        for instruction in &self.instructions {
            for _ in 0..instruction.take {
                if let Some(v) = stacks[instruction.from - 1].pop() {
                    stacks[instruction.to - 1].push(v);
                }
            }

        }

        stacks.into_iter().map(|v| v.last().copied().unwrap_or(' ')).collect()
    }

    fn part2(&self) -> String {
        let mut stacks = self.stacks.clone();

        for instruction in &self.instructions {
            let take_from = stacks[instruction.from-1].len() - instruction.take;
            let arr = stacks[instruction.from - 1].split_off(take_from);
            stacks[instruction.to - 1].extend(arr);
        }

        stacks.into_iter().map(|v| v.last().copied().unwrap_or(' ')).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn part1() {
        let res = SupplyStacks::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), "CMZ");
    }

    #[test]
    fn part2() {
        let res = SupplyStacks::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), "MCD");
    }
}
