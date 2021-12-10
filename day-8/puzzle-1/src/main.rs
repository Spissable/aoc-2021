use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut counter: usize = 0;
        let unique_lengths = [2,3,4,7];

        for line in lines {
            if let Ok(line) = line {
                line.split("|").collect::<Vec<&str>>()[1].split_whitespace().for_each(|output| {
                    if unique_lengths.contains(&output.len()) {
                       counter += 1; 
                    } 
                });
            }
        }
        
        println!("{}", counter);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
