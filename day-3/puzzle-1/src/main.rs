use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Diagnostic {
    bits: Vec<Vec<bool>>,
}

impl Diagnostic {
    fn new(size: u32) -> Diagnostic {
        return Diagnostic {
            bits: vec![vec![]; size as usize],
        }
    }

    fn parse_line(&mut self, line: String) {
        for (index, char) in line.chars().enumerate() {
            self.bits[index].push(char.to_digit(2).unwrap() != 0);
        }
    }

    fn get_result(&self) -> usize {
        let mut gamma = String::from("");
        let mut epsilon = String::from("");
        
        for column in self.bits.iter() {
            let zeros = column.iter().filter(|&&b| b == false).collect::<Vec<&bool>>().len();
            let ones = column.len() - zeros;
            gamma += &format!("{}", (ones > zeros) as u32);
            epsilon += &format!("{}", (ones < zeros) as u32);
        }

        let gamma = usize::from_str_radix(gamma.as_str(), 2).unwrap();
        let epsilon = usize::from_str_radix(epsilon.as_str(), 2).unwrap();

        return gamma * epsilon;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut diagnostic = Diagnostic::new(12);

        for line in lines {
            if let Ok(line) = line {
                diagnostic.parse_line(line);
            }
        }

        println!("{}", diagnostic.get_result());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
