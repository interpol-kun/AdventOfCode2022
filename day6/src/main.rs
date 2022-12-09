use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{:?}", solve().unwrap_or_default());
}

fn solve() -> Result<usize, std::io::Error> {
    let line = BufReader::new(File::open("src/input")?);
    let line = line.lines().next().unwrap()?.into_bytes();
    let needle_size = 14;

    let answer = line
        .windows(needle_size)
        .enumerate()
        .take_while(|(_, str)| !unique(str))
        .last()
        .unwrap()
        .0;

    Ok(answer + needle_size + 1)
}

fn unique(v: &[u8]) -> bool {
    let mut uniques = HashSet::new();
    for f in v.iter() {
        if !uniques.insert(*f) {
            return false;
        }
    }
    true
}
