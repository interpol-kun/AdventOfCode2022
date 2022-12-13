//based on https://github.com/gbegerow solution

use regex::Regex;

fn main() {
    println!("{:?}", solve(include_str!("input")));
    println!("{:?}", solve_second(include_str!("input")));
}

#[derive(Debug)]
struct Directory {
    name: String,
    size: usize,
    parent: usize,
}

fn parse(input: &str) -> Vec<Directory> {
    let mut directories = vec![Directory {
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }];

    let root = 0;
    let mut cwd = root;

    let dir_rx = Regex::new(r"\s*\$ cd (?P<target>.+)").unwrap();
    let file_rx = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

    for line in input.lines() {
        if let Some(cap) = dir_rx.captures(line) {
            match &cap["target"] {
                "/" => cwd = root,
                ".." => cwd = directories[cwd].parent,
                name => {
                    directories.push(Directory {
                        name: String::from(name),
                        size: 0,
                        parent: cwd,
                    });
                    cwd = directories.len() - 1;
                }
            }
        } else if let Some(cap) = file_rx.captures(line) {
            let size = &cap["size"].parse().unwrap();

            let mut p = cwd;
            loop {
                directories[p].size += size;
                if p == root {
                    break;
                }
                p = directories[p].parent;
            }
        }
    }

    directories
}

fn solve(input: &str) -> usize {
    let filesystem = parse(input);

    filesystem
        .iter()
        .map(|f| f.size)
        .filter(|size| *size <= 100_000)
        .sum()
}

fn solve_second(input: &str) -> usize {
    let filesystem = parse(input);
    let free_space = 70_000_000 - filesystem[0].size;
    let need_to_free = 30_000_000 - free_space;

    let _size = filesystem[0].size;

    filesystem
        .iter()
        .map(|f| f.size)
        .filter(|size| *size >= need_to_free)
        .min()
        .expect("at least one directory must be deleted")
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_07_a_example() {
        assert_eq!(super::solve(TEST_INPUT), 95437);
    }

    #[test]
    fn aoc_2022_07_b_example() {
        assert_eq!(super::solve_second(TEST_INPUT), 24933642);
    }

    const TEST_INPUT: &str = "
$ cd /
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
7214296 k";
}
