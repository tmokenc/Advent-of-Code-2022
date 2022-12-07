use std::collections::HashMap;

#[derive(Debug)]
struct DirEntry {
    size: Option<usize>,
    name: String,
}

impl DirEntry {
    fn from_str(s: &str) -> Option<Self> {
        let mut iter = s.split(" ");
        let size = match iter.next() {
            Some("dir") => None,
            Some(v) => Some(v.parse::<usize>().ok()?),
            None => return None,
        };

        Some(Self {
            size,
            name: iter.collect::<String>(),
        })
    }
}

enum Command {
    Cd { arg: String },
    Ls { output: Vec<DirEntry> }
}

#[derive(Default)]
pub struct NoSpaceLeftOnDevice {
    cmds: Vec<Command>,
}

impl NoSpaceLeftOnDevice {
    fn file_list(&self) -> HashMap<Vec<String>, usize> {
        let mut res = HashMap::new();
        let mut current_dir = Vec::new();
        for cmd in &self.cmds {
            match cmd {
                Command::Cd { arg } => {
                    match arg.as_str() {
                        "/" => current_dir.clear(),
                        ".." => { current_dir.pop(); }
                        v => current_dir.push(v.to_owned()),
                    }
                }

                Command::Ls { output } => {
                    for entry in output {
                        if let Some(size) = entry.size {
                            let mut path = current_dir.to_owned();
                            path.push(entry.name.to_owned());
                            res.insert(path, size);
                        }
                    }
                }
            }
        }

        res
    }

    fn dir_list(&self) -> HashMap<Vec<String>, usize> {
        let mut list = HashMap::new();

        for (mut path, size) in self.file_list() {
            path.pop();
            while !path.is_empty() {
                list.entry(path.to_owned()).and_modify(|v| *v += size).or_insert(size);
                path.pop();
            }

            list.entry(Vec::default()).and_modify(|v| *v += size).or_insert(size);
        }

        list
    }
}

impl crate::AdventOfCode<usize> for NoSpaceLeftOnDevice {
    const TITLE: &'static str = "No Space Left On Device";
    const DAY: u8 = 7;

    fn new(input: &str) -> Option<Self> {
        let mut iter = input.lines().peekable();
        let mut res = Self::default();

        while let Some(line) = iter.next() {
            let mut args = line.split_whitespace();
            args.next().filter(|&v| v == "$")?;

            let cmd = match args.next()? {
                "cd" => Command::Cd {
                    arg: args.next()?.to_owned(),
                },
                "ls" => {
                    let mut output = Vec::new();

                    while let Some(line) = iter.peek() {
                        if line.starts_with("$ ") {
                            break;
                        }

                        let entry = DirEntry::from_str(iter.next()?)?;
                        output.push(entry);
                    }

                    Command::Ls { output }
                }
                _ => return None,
            };

            res.cmds.push(cmd);
        }

        Some(res)
    }

    fn part1(&self) -> usize {
        self.dir_list().into_values().filter(|&v| v  < 100000).sum()
    }

    fn part2(&self) -> usize {
        const TOTAL_SPACE: usize = 70000000;
        const NEEDED_SPACE: usize = 30000000;

        let dir_list = self.dir_list();
        let free_space = TOTAL_SPACE - dir_list.get(&Vec::default()).unwrap();
        let mut min = usize::MAX;

        for (_, size) in dir_list {
            if size + free_space >= NEEDED_SPACE {
                if size < min {
                    min = size;
                }
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    const EXAMPLE_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn part1() {
        let res = NoSpaceLeftOnDevice::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part1(), 95437);
    }

    #[test]
    fn part2() {
        let res = NoSpaceLeftOnDevice::new_unwrap(EXAMPLE_INPUT);
        assert_eq!(res.part2(), 24933642);
    }
}
