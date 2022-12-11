use std::io::{self, Write};

struct CPU<T: Iterator<Item=Instruction>>{
    x: isize,
    current_operation: Option<isize>,
    instructions: T,
}

impl<T: Iterator<Item=Instruction>> CPU<T> {
    fn new(instructions: T) -> Self {
        Self {
            x: 1,
            current_operation: None,
            instructions,
        }
    }

    fn tick(&mut self) -> isize {
        if let Some(val) = self.current_operation.take() {
            let old = self.x;
            self.x += val;
            return old;
        }

        if let Some(Instruction::Addx(val)) = self.instructions.next() {
            self.current_operation.replace(val);
        }

        self.x
    }

    fn draw_row(&mut self, mut output: impl Write) {
        for current_position in 1..=40 {
            let position = self.tick();

            if (position..(position+3)).contains(&current_position) {
                output.write(b"#").ok();
            } else {
                output.write(b".").ok();
            }
        }

        output.write(b"\n").ok();
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(isize),
}

#[derive(Default)]
pub struct CathodeRayTube {
    instructions: Vec<Instruction>
}

impl crate::AdventOfCode<isize> for CathodeRayTube {
    const TITLE: &'static str = "Cathode-Ray Tube";
    const DAY: u8 = 10;

    fn new(input: &str) -> Option<Self> {
        let mut res = Self::default();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let instruction = match iter.next()? {
                "noop" => Instruction::Noop,
                "addx" => {
                    let val = iter.next()?.parse::<isize>().ok()?;
                    Instruction::Addx(val)
                }
                _ => return None,
            };

            res.instructions.push(instruction);
        }

        Some(res)
    }

    fn part1(&self) -> isize {
        let mut cpu = CPU::new(self.instructions.iter().copied());
        let mut res = 0;

        res += 20  * (0..20).map(|_| cpu.tick()).last().unwrap();
        res += 60  * (0..40).map(|_| cpu.tick()).last().unwrap();
        res += 100 * (0..40).map(|_| cpu.tick()).last().unwrap();
        res += 140 * (0..40).map(|_| cpu.tick()).last().unwrap();
        res += 180 * (0..40).map(|_| cpu.tick()).last().unwrap();
        res += 220 * (0..40).map(|_| cpu.tick()).last().unwrap();

        res
    }

    fn part2(&self) -> isize {
        let mut cpu = CPU::new(self.instructions.iter().copied());

        for _ in 0..6 {
            cpu.draw_row(io::stdout());
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn part1() {
        let res = CathodeRayTube::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 13140);
    }

    #[test]
    fn part2_1() {
        let res = CathodeRayTube::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 0);
    }
}
