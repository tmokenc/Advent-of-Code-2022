pub struct RucksackReorganization {
    rucksacks: Vec<String>,
}

fn char_to_idx(ch: char) -> usize {
    match ch {
        'a'..='z' => ch as usize - 'a' as usize + 1,
        'A'..='Z' => ch as usize - 'A' as usize + 27,
        _ => 0,
    }
}

impl crate::AdventOfCode for RucksackReorganization {
    const TITLE: &'static str = "Rucksack Reorganization";
    const DAY: u8 = 3;

    fn new(input: &str) -> Option<Self> {
        let rucksacks = crate::utils::lines_to_vec::<String>(input)?;
        Some(Self { rucksacks })
    }

    fn part1(&self) -> u64 {
        let mut priorities: u64 = 0;

        for rucksack in &self.rucksacks {
            let mut list = [false; 53];
            let mut list2 = [false; 53];
            let middle_point = rucksack.len() / 2;

            for ch in rucksack.get(..middle_point).unwrap().chars() {
                let idx = char_to_idx(ch);
                list[idx] = true;
            }

            for ch in rucksack.get(middle_point..).unwrap().chars() {
                let idx = char_to_idx(ch);
                list2[idx] = true;
            }

            priorities += (1..=52).filter(|&v| list[v] && list2[v]).sum::<usize>() as u64;
        }

        priorities
    }

    fn part2(&self) -> u64 {
        let mut priorities: u64 = 0;

        let mut iter = self.rucksacks.iter();

        while let Some(rucksack) = iter.next() {
            let mut list = [false; 53];
            let mut list2 = [false; 53];
            let mut list3 = [false; 53];

            for ch in rucksack.chars() {
                list[char_to_idx(ch)] = true;
            }

            for ch in iter.next().unwrap().chars() {
                list2[char_to_idx(ch)] = true;
            }

            for ch in iter.next().unwrap().chars() {
                list3[char_to_idx(ch)] = true;
            }

            priorities += (1..=52).filter(|&v| list[v] && list2[v] && list3[v]).sum::<usize>() as u64;
        }

        priorities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part1() {
        let res = RucksackReorganization::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 157);
    }

    #[test]
    fn part2() {
        let res = RucksackReorganization::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 70);
    }
}
