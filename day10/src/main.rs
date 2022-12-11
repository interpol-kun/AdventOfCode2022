use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{:?}", solve().unwrap_or_default());
}

fn solve() -> Result<i32, std::io::Error> {
    let lines = BufReader::new(File::open("src/input")?).lines();

    let mut cycles: usize = 0;
    let mut reg_x: i32 = 1;
    let mut signal_strength = 0;

    let symbol = 'X';

    let markers = vec![20, 60, 100, 140, 180, 220];

    let mut screen = vec!['.'; 6 * 40];

    for line in lines {
        let line = line?;
        let line: Vec<&str> = line.split_whitespace().collect();

        match line[0] {
            "noop" => {
                cycles += 1;

                if (cycles - 1) % 40 >= reg_x as usize - 1
                    && (cycles - 1) % 40 <= reg_x as usize + 1
                {
                    screen[cycles - 1] = symbol;
                }

                if markers.contains(&cycles) {
                    signal_strength += cycles * reg_x as usize;
                }
            }
            "addx" => {
                for _i in 0..2 {
                    cycles += 1;

                    if (cycles - 1) % 40 >= reg_x as usize - 1
                        && (cycles - 1) % 40 <= reg_x as usize + 1
                    {
                        screen[cycles - 1] = symbol;
                    }

                    if markers.contains(&cycles) {
                        signal_strength += cycles * reg_x as usize;
                    }
                }
                reg_x += line[1].parse::<i32>().unwrap_or_default();
            }
            _ => continue,
        }
    }

    for row in screen.chunks(40) {
        print!("{:?}", row);
        println!();
    }

    Ok(signal_strength as i32)
}
