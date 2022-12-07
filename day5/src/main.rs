use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{:?}", solve().unwrap_or_default());
}

fn solve() -> Result<String, std::io::Error> {
    let mut lines = BufReader::new(File::open("src/input")?).lines();

    let mut cargos: Vec<Vec<char>> = Vec::new();

    if let Some(line) = lines.next() {
        let line = line?;
        let stack_num = line.len() / 4 + 1;
        let value: Vec<char> = Vec::new();
        cargos.resize(stack_num, value);

        let mut idx = 0;

        for char in line.chars().enumerate() {
            if char.0 % 4 == 1 && char.1 != ' ' {
                cargos[idx].push(char.1);
                idx += 1;
            } else if char.0 % 4 == 1 {
                idx += 1;
            }
        }
    }

    let mut prepared = false;
    for line in lines.by_ref() {
        let line = line?;

        let mut idx = 0;

        for char in line.chars().enumerate() {
            if char.1.is_numeric() {
                prepared = true;
                break;
            }
            if char.0 % 4 == 1 && char.1 != ' ' {
                cargos[idx].push(char.1);
                idx += 1;
            } else if char.0 % 4 == 1 {
                idx += 1;
            }
        }

        if prepared {
            break;
        }
    }

    lines.next();

    for cargo in &mut cargos {
        cargo.reverse();
    }

    println!("{:?}", cargos);

    for line in lines {
        let line = line?;

        let stuff: Vec<&str> = line.split_whitespace().collect();

        let what = stuff[1].parse::<u32>().unwrap_or_default();
        let from = stuff[3].parse::<u32>().unwrap_or_default() - 1;
        let to = stuff[5].parse::<u32>().unwrap_or_default() - 1;

        let mut load: Vec<char> = Vec::new();
        for _ in 0..what {
            load.push(cargos[from as usize].pop().unwrap());
        }

        let second = true;
        if !second {
            load.reverse();
        }

        for _ in 0..what {
            cargos[to as usize].push(load.pop().unwrap());
        }
    }

    let mut result = String::new();
    for cargo in cargos {
        result.push(cargo.last().unwrap().to_owned())
    }

    Ok(result)
}
