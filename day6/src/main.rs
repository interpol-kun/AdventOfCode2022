use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{:?}", solve().unwrap_or_default());
}

fn solve() -> Result<i32, std::io::Error> {
    let mut line = BufReader::new(File::open("src/input")?).lines();
    let line = line.next().unwrap()?;

    let mut sequence = Vec::new();
    let load_size = 14;

    for c in line.chars().enumerate() {
        sequence.push(c.1);

        let is_unique = unique(&mut sequence);

        if sequence.len() == load_size && is_unique {
            println!("Found {:?}", sequence);
            return Ok(c.0 as i32 + 1);
        } else if sequence.len() == load_size {
            _ = sequence.remove(0);
        }
    }
    Ok(0)
}

fn unique<T: Eq + Hash + Copy>(v: &mut [T]) -> bool {
    let mut uniques: HashSet<T> = HashSet::new();
    for f in v.iter() {
        if !uniques.insert(*f) {
            return false;
        }
    }
    true
}
