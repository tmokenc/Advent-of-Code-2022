pub struct TuningTrouble {
    buffer: Vec<char>,
}

fn tuning_trouble<const N: usize>(buffer: &[char]) -> usize {
    let mut count = N;
    let mut chars: [char; N] = buffer[..N].try_into().unwrap();

    while !is_unique_arr(&chars) {
        let Some(ch) = buffer.get(count) else {
            break;
        };

        chars[count % N] = *ch;
        count += 1;
    }

    count
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

impl crate::AdventOfCode<usize> for TuningTrouble {
    const TITLE: &'static str = "Tuning Trouble";
    const DAY: u8 = 6;

    fn new(input: &str) -> Option<Self> {
        Some(Self {
            buffer: input.chars().collect(),
        })
    }

    fn part1(&self) -> usize {
        tuning_trouble::<4>(&self.buffer)
    }

    fn part2(&self) -> usize {
        tuning_trouble::<14>(&self.buffer)
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
