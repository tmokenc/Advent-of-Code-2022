use std::collections::HashSet;

type Coor = (isize, isize);

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn modification(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

fn tail_move(tail: &mut Coor, head: Coor) {
    let x_distance = head.0 - tail.0;
    let y_distance = head.1 - tail.1;
    
    if x_distance > 1 {
        tail.0 += 1;
        tail.1 += y_distance.clamp(-1, 1);
    } else if x_distance < -1 {
        tail.0 -= 1;
        tail.1 += y_distance.clamp(-1, 1);
    } else if y_distance > 1 {
        tail.1 += 1;
        tail.0 += x_distance.clamp(-1, 1);
    } else if y_distance < -1 {
        tail.1 -= 1;
        tail.0 += x_distance.clamp(-1, 1);
    }
}

#[derive(Default)]
pub struct RopeBridge {
    motions: Vec<(Direction, isize)>
}

impl crate::AdventOfCode<usize> for RopeBridge {
    const TITLE: &'static str = "Rope Bridge";
    const DAY: u8 = 9;

    fn new(input: &str) -> Option<Self> {
        let mut res = Self::default();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let direction = match iter.next()? {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => return None,
            };

            let val = iter.next()?.parse::<isize>().ok()?;
            iter.next().is_none().then_some(())?;
            res.motions.push((direction, val));
        }

        Some(res)
    }

    fn part1(&self) -> usize {
        let mut visited = HashSet::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);
        
        for (direction, val) in self.motions.iter().copied() {
            for _ in 0..val {
                let (x_mod, y_mod) = direction.modification();
                head.0 += x_mod;
                head.1 += y_mod;
                tail_move(&mut tail, head);
                visited.insert(tail);
            }
        }

        visited.len()
    }

    fn part2(&self) -> usize {
        let mut visited = HashSet::new();
        let mut head = (0, 0);
        let mut tails = [(0, 0); 9];
        
        for (direction, val) in self.motions.iter().copied() {
            for _ in 0..val {
                let (x_mod, y_mod) = direction.modification();
                head.0 += x_mod;
                head.1 += y_mod;

                let mut current_head = head;

                for tail in tails.iter_mut() {
                    tail_move(tail, current_head);
                    current_head = *tail;
                }

                visited.insert(current_head);
            }
        }

        visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn part1() {
        let res = RopeBridge::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 13);
    }

    #[test]
    fn part2_1() {
        let res = RopeBridge::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 1);
    }

    #[test]
    fn part2_2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        let res = RopeBridge::new_unwrap(input);
        assert_eq!(res.part2(), 36);
    }
}
