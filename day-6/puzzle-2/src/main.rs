use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lanternfishes: [u64; 9] = [0;9];

        for line in lines {
            if let Ok(line) = line {
                line.split(",").for_each(|t| lanternfishes[t.parse::<usize>().unwrap()] +=1);
            }
        }
        
        let days = 256;
        for _ in 0..days {
            lanternfishes.rotate_left(1);
            lanternfishes[6] += lanternfishes[8];
        }
        println!("{}", lanternfishes.iter().sum::<u64>());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
