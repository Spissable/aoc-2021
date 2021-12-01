use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut prev_depth: Option<u32> = None;
        let mut larger_count: u32 = 0;

        for line in lines {
            if let Ok(current_depth) = line {
                let current_depth = current_depth.parse::<u32>().unwrap();

                if let Some(prev) = prev_depth {
                    if current_depth > prev {
                        larger_count += 1;
                    }
                }

                prev_depth = Some(current_depth);
            }
        }

        println!("{}", larger_count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
