use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{:?}", solve_second().unwrap_or(0));
}

fn solve() -> Result<u32, std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let mut result: u32 = 0;

    for line in lines {
        let rucksack = line?.to_string().chars().collect::<Vec<_>>();
        let compartments = rucksack.split_at(rucksack.len() / 2);

        let mut diff: Vec<&char> = compartments
            .0
            .iter()
            .filter(|k| compartments.1.contains(k))
            .collect();

        diff.dedup();

        let mut sum: u32 = 0;

        for item in &diff {
            if item.is_uppercase() {
                sum += (**item as u32 & 0x1F) + 26;
            } else {
                sum += **item as u32 & 0x1F;
            }
        }

        //println!("{:?} {:?}", sum, diff);

        result += sum;
    }
    Ok(result)
}

fn solve_second() -> Result<u32, std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let mut result: u32 = 0;

    let mut rucksacks: Vec<Vec<char>> = Vec::new();

    for line in lines {
        rucksacks.push(line?.to_string().chars().collect::<Vec<_>>());

        if rucksacks.len() > 2 {
            let mut diff: Vec<&char> = rucksacks[0]
                .iter()
                .filter(|k| rucksacks[1].contains(k) && rucksacks[2].contains(k))
                .collect();

            diff.dedup();
            let mut sum: u32 = 0;

            for item in &diff {
                if item.is_uppercase() {
                    sum += (**item as u32 & 0x1F) + 26;
                } else {
                    sum += **item as u32 & 0x1F;
                }
            }

            println!("{:?} {:?}", sum, diff);

            result += sum;

            rucksacks.clear()
        }
    }
    Ok(result)
}
