enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Default)]
pub struct TreetopTreeHouse {
    map: Vec<Vec<u8>>,
}

impl TreetopTreeHouse {
    fn is_tree_visible(&self, x: usize, y: usize, direction: Direction) -> bool {
        let (mod_x, mod_y) = direction.modification();
        let height = self.map[x][y];
        let x_len = self.map.len() as isize;
        let y_len = self.map[0].len() as isize;
        let mut x = x as isize + mod_x;
        let mut y = y as isize + mod_y;

        while x >= 0 && y >= 0 && x < x_len && y < y_len {
            if self.map[x as usize][y as usize] >= height {
                return false;
            }

            x += mod_x;
            y += mod_y;
        }

        true
    }

    fn scenic_score(&self, x: usize, y: usize, direction: Direction) -> usize {
        let (mod_x, mod_y) = direction.modification();
        let height = self.map[x][y];
        let x_len = self.map.len() as isize;
        let y_len = self.map[0].len() as isize;
        let mut x = x as isize + mod_x;
        let mut y = y as isize + mod_y;
        let mut score = 0;

        while x >= 0 && y >= 0 && x < x_len && y < y_len {
            score += 1;

            if self.map[x as usize][y as usize] >= height {
                break;
            }

            x += mod_x;
            y += mod_y;
        }

        score
    }
}

impl crate::AdventOfCode<usize> for TreetopTreeHouse {
    const TITLE: &'static str = "Treetop Tree House";
    const DAY: u8 = 8;

    fn new(input: &str) -> Option<Self> {
        let mut map = Vec::new();

        for line in input.lines() {
            let row = line
                .chars()
                .filter_map(|v| v.to_digit(10))
                .map(|v| v as u8)
                .collect();

            map.push(row);
        }

        Some(Self { map })
    }

    fn part1(&self) -> usize {
        let x_len = self.map.len();
        let y_len = self.map[0].len();
        let mut visible_trees = ((x_len + y_len) * 2) - 4;

        for x in 1..(x_len - 1) {
            for y in 1..(y_len - 1) {
                let directions = [
                    Direction::Up, 
                    Direction::Down, 
                    Direction::Left, 
                    Direction::Right
                ];

                if directions.into_iter().any(|v| self.is_tree_visible(x, y, v)) {
                    visible_trees += 1;
                }
            }
        }

        visible_trees
    }

    fn part2(&self) -> usize {
        let x_len = self.map.len();
        let y_len = self.map[0].len();
        let mut max = usize::MIN;

        for x in 1..(x_len - 1) {
            for y in 1..(y_len - 1) {
                let directions = [
                    Direction::Up, 
                    Direction::Down, 
                    Direction::Left, 
                    Direction::Right
                ];

                let score = directions.into_iter().map(|v| self.scenic_score(x, y, v)).product();
                max = std::cmp::max(max, score);
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn part1() {
        let res = TreetopTreeHouse::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 21);
    }

    #[test]
    fn part2() {
        let res = TreetopTreeHouse::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 8);
    }
}
