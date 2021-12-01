use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let window_size = 3;
        let mut larger_count = 0;
        let mut measurements: Vec<u32> = vec![];
        for line in lines {
            if let Ok(depth) = line {
                measurements.push(depth.parse::<u32>().unwrap());
            }
        }

        for (index, _) in measurements.iter().enumerate() {
            if is_bigger(&measurements, index, window_size) {
                larger_count += 1;
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

fn is_bigger(measurements: &Vec<u32>, index: usize, size: usize) -> bool {
    if measurements.len() <= index + size {
        return false;
    }

    let a: u32 = measurements[index..index + size].iter().sum();
    let b: u32 = measurements[index + 1..index + 1 + size].iter().sum();

    return b > a;
}
