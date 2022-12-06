use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    println!("{:?}", solve_second().unwrap_or_default());
}

fn solve() -> Result<i32, std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let mut count = 0;

    for line in lines {
        let line = line?;
        let pairs: Vec<_> = line.split(',').collect();

        let first_range: Vec<_> = pairs[0]
            .split('-')
            .map(|f| f.parse::<i32>().unwrap_or_default())
            .collect();

        let second_range: Vec<_> = pairs[1]
            .split('-')
            .map(|f| f.parse::<i32>().unwrap_or_default())
            .collect();

        if first_range[0] >= second_range[0] && first_range[1] <= second_range[1] {
            println!("{:?}", first_range);
            count += 1;
            continue;
        }

        if second_range[0] >= first_range[0] && second_range[1] <= first_range[1] {
            println!("{:?}", second_range);
            count += 1;
            continue;
        }
    }

    Ok(count)
}

fn solve_second() -> Result<i32, std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let mut count = 0;

    for line in lines {
        let line = line?;
        let pairs: Vec<_> = line.split(',').collect();

        let first_range: Vec<_> = pairs[0]
            .split('-')
            .map(|f| f.parse::<i32>().unwrap_or_default())
            .collect();

        let second_range: Vec<_> = pairs[1]
            .split('-')
            .map(|f| f.parse::<i32>().unwrap_or_default())
            .collect();

        if first_range[0] <= second_range[1] && first_range[1] >= second_range[0] {
            println!("{:?}", first_range);
            count += 1;
            continue;
        }
    }

    Ok(count)
}
