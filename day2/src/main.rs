use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Your score is: {:?}", solve().unwrap_or_default());
}

fn solve() -> Result<i32, std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let table_first = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let table_second = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    let mut score = 0;

    for line in lines {
        let key = line?.to_string();

        score += table_second.get(&key.as_ref()).unwrap_or(&0);
    }

    Ok(score)
}
