use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let case = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&"0".to_string())
        .parse::<usize>()
        .unwrap_or_default();

    println!(
        "Some elf has the most calories: {:?}",
        calculate(case).unwrap_or_default()
    );
}

fn calculate(case: usize) -> Result<i32, std::io::Error> {
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

    match case {
        1 => Result::Ok(elves[elves.len() - 3..elves.len()].iter().sum::<i32>()),
        _ => Result::Ok(elves.pop().unwrap_or_default()),
    }
}
