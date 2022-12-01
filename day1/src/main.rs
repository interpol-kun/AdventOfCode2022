use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let mut elves: Vec<i32> = vec![0];
    let mut index = 0;

    for line in lines {
        let item = line?.to_string();

        if !item.is_empty() {
            elves[index] += item.parse::<i32>().unwrap_or_default();
        } else {
            elves.push(0);
            index += 1;
        }
    }

    elves.sort();

    println!(
        "Some elf has the most calories: {:?}",
        elves.last().unwrap()
    );

    Ok(())
}
