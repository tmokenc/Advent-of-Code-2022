mod day01;
mod day02;

pub mod utils;
pub use utils::*;

use humantime::format_duration;
use owo_colors::OwoColorize as _;
use std::env;
use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

pub trait AdventOfCode<Output = u64>
where
    Output: Display,
{
    const TITLE: &'static str;
    const DAY: u8;

    fn new(input: &str) -> Option<Self>
    where
        Self: Sized;

    fn part1(&self) -> Output;
    fn part2(&self) -> Output;
    fn new_unwrap(input: &str) -> Self
    where
        Self: Sized,
    {
        match Self::new(input) {
            Some(res) => res,
            None => panic!(
                "Cannot parse the input of `{}` (day {})",
                Self::TITLE,
                Self::DAY
            ),
        }
    }
}

#[derive(Clone, Copy)]
struct Timing {
    title: &'static str,
    parsing: Duration,
    part1: Duration,
    part2: Duration,
}

fn time<T>(f: impl Fn() -> T) -> (T, Duration) {
    let start = Instant::now();
    let res = f();
    let duration = start.elapsed();

    (res, duration)
}

fn exec_once<AoC: AdventOfCode + 'static>(input: String) -> Option<Timing> {
    let (f, parse_time) = time(|| AoC::new(input.trim()));

    let f = match f {
        Some(f) => f,
        None => {
            println!("Cannot parse the input");
            return None;
        }
    };

    println!(
        "Input parsed in {}",
        format_duration(parse_time).bright_magenta()
    );

    let (res, part1_time) = time(|| f.part1());
    println!(
        "Part 1: {} ({}) (total {})",
        res,
        format_duration(part1_time).cyan(),
        format_duration(part1_time + parse_time).bright_cyan(),
    );

    let (res, part2_time) = time(|| f.part2());
    println!(
        "Part 2: {} ({}) (total {})",
        res,
        format_duration(part2_time).cyan(),
        format_duration(part2_time + parse_time).bright_cyan(),
    );

    Some(Timing {
        title: AoC::TITLE,
        parsing: parse_time,
        part1: part1_time,
        part2: part2_time,
    })
}

fn exec<AoC: AdventOfCode + 'static>() -> Option<Timing> {
    let title = format!("DAY {} - {}", AoC::DAY, AoC::TITLE);
    println!("{}", title.bold());

    let input_name = format!("day{:02}.txt", AoC::DAY);
    let input_path = Path::new("./input/").join(&input_name);

    // println!("Solution");
    let input = fs::read_to_string(input_path).ok()?;
    exec_once::<AoC>(input)
}

fn run(day: u8) -> Option<Timing> {
    match day {
        01 => exec::<day01::CalorieCounting>(),
        02 => exec::<day02::RockPaperScissors>(),
        26.. => {
            println!("{day} is not a valid day for AdventOfCode");
            None
        }
        _ => {
            println!("There is no solution for day {day} yet");
            None
        }
    }
}

fn main() {
    if env::args().any(|v| v == "-l" || v == "--log") {
        setup_logger();
    }

    let mut days = env::args()
        .skip(1)
        .filter_map(|v| v.parse::<u8>().ok())
        .collect::<Vec<_>>();

    if days.is_empty() {
        days = (1..=25).collect();
    }

    let mut timings: [Option<Timing>; 25] = [None; 25];
    let mut iter = days.into_iter().peekable();

    while let Some(day) = iter.next() {
        let timing = run(day);
        timings[day as usize - 1] = timing;

        if timing.is_some() && iter.peek().is_some() {
            println!("");
        }
    }

    if env::args().any(|v| v == "--benchmark" || v == "-b") {
        print_benchmark(timings).unwrap();
    }
}

fn setup_logger() -> () {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("{} - {}", record.level(), msg)))
        .level(log::LevelFilter::Debug)
        .chain(fs::File::create("output.log").unwrap())
        .apply()
        .unwrap()
}

fn display_benchmark_time(time: Duration) -> String {
    humantime::format_duration(time)
        .to_string()
        .split_inclusive(' ')
        .take(2)
        .collect()
}

fn print_benchmark(timings: [Option<Timing>; 25]) -> std::io::Result<()> {
    let mut file = fs::File::create("benchmark.md")?;

    writeln!(
        &mut file,
        "| Day - Name | Parse time | Part 1 | Part 2 | AoC link |"
    )?;
    writeln!(
        &mut file,
        "| :--------- | ---------: | -----: | -----: | :------: |"
    )?;

    for (timing, day) in timings.iter().zip(1..) {
        let url = "[🔗](https://adventofcode.com/2020/day/{day})";

        if let Some(timing) = timing {
            writeln!(
                &mut file,
                "| [{:02} - {}](/src/day{:02}.rs) | {} | {} | {} | {url} |",
                day,
                timing.title,
                day,
                display_benchmark_time(timing.parsing),
                display_benchmark_time(timing.part1),
                display_benchmark_time(timing.part2),
            )?;
        } else {
            writeln!(&mut file, "| {:02} - | - | - | - | {url} |", day)?;
        }
    }

    Ok(())
}
