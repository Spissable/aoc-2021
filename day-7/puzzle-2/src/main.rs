use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut positions: HashMap<i64, i64> = HashMap::new();
        let mut biggest_pos = 0;

        for line in lines {
            if let Ok(line) = line {
                line.split(",").for_each(|str| {
                    let num: i64 = str.parse().unwrap();

                    if num > biggest_pos {
                        biggest_pos = num;
                    }

                    if positions.contains_key(&num) {
                        positions.insert(num, positions.get(&num).unwrap() + 1);
                    } else {
                        positions.insert(num, 1);
                    }
                });
            }
        }

        let mut ideal_step_count: Option<i64> = None;
        for i in 0..biggest_pos {
            let mut current_step_count = 0;
            positions.iter().for_each(|(pos, amount)| {
                let fuel = (i - pos).abs();
                current_step_count += fuel * (fuel + 1) / 2 * amount;
            });

            match ideal_step_count {
                Some(ideal) => {
                    if current_step_count < ideal {
                        ideal_step_count = Some(current_step_count);
                    }
                }
                None => {
                    ideal_step_count = Some(current_step_count);
                }
            };
        }

        println!("{}", ideal_step_count.unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
