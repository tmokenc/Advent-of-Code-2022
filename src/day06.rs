pub struct TuningTrouble {
    buffer: Vec<char>,
}

fn is_unique_arr(arr: &[char]) -> bool {
    for i in 0..(arr.len()-1) {
        for j in (i+1)..arr.len() {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }

    true
}

impl crate::AdventOfCode for TuningTrouble {
    const TITLE: &'static str = "Tuning Trouble";
    const DAY: u8 = 6;

    fn new(input: &str) -> Option<Self> {
        Some(Self {
            buffer: input.chars().collect(),
        })
    }

    fn part1(&self) -> u64 {
        let mut count = 4;
        let mut chars: [char; 4] = self.buffer[..4].try_into().unwrap();

        while !is_unique_arr(&chars) {
            let Some(ch) = self.buffer.get(count) else {
                break;
            };

            chars[count % 4] = *ch;
            count += 1;
        }

        count as u64
    }

    fn part2(&self) -> u64 {
        let mut count = 14;
        let mut chars: [char; 14] = self.buffer[..14].try_into().unwrap();

        while !is_unique_arr(&chars) {
            let Some(ch) = self.buffer.get(count) else {
                break;
            };

            chars[count % 14] = *ch;
            count += 1;
        }

        count as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_INPUT_2: &str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_INPUT_3: &str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_INPUT_4: &str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_INPUT_5: &str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 7);
    }

    #[test]
    fn part1_2() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_2);
        assert_eq!(res.part1(), 5);
    }

    #[test]
    fn part1_3() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_3);
        assert_eq!(res.part1(), 6);
    }

    #[test]
    fn part1_4() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_4);
        assert_eq!(res.part1(), 10);
    }

    #[test]
    fn part1_5() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_5);
        assert_eq!(res.part1(), 11);
    }

    #[test]
    fn part2() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 19);
    }

    #[test]
    fn part2_2() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_2);
        assert_eq!(res.part2(), 23);
    }

    #[test]
    fn part2_3() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_3);
        assert_eq!(res.part2(), 23);
    }

    #[test]
    fn part2_4() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_4);
        assert_eq!(res.part2(), 29);
    }

    #[test]
    fn part2_5() {
        let res = TuningTrouble::new_unwrap(EXAMPLE_INPUT_5);
        assert_eq!(res.part2(), 26);
    }
}
