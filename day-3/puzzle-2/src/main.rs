use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Diagnostic {
    bits: Vec<Vec<bool>>,
}

impl Diagnostic {
    fn new() -> Diagnostic {
        return Diagnostic { bits: vec![] };
    }

    fn parse_line(&mut self, line: String) {
        self.bits.push(
            line.chars()
                .map(|char| char.to_digit(2).unwrap() != 0)
                .collect(),
        );
    }
}

fn get_oxygen_rating(ratings: Vec<Vec<bool>>, column: usize) -> Vec<Vec<bool>> {
    if ratings.len() == 1 {
        return ratings;
    }

    let zeros: Vec<Vec<bool>> = ratings
        .clone()
        .into_iter()
        .filter(|rating| rating[column] == false)
        .collect();
    let ones: Vec<Vec<bool>> = ratings
        .clone()
        .into_iter()
        .filter(|rating| rating[column] == true)
        .collect();

    let remaining = if zeros.len() > ratings.len() / 2 {
        zeros
    } else {
        ones
    };

    return get_oxygen_rating(remaining, column + 1);
}

fn get_co2_rating(ratings: Vec<Vec<bool>>, column: usize) -> Vec<Vec<bool>> {
    if ratings.len() == 1 {
        return ratings;
    }

    let zeros: Vec<Vec<bool>> = ratings
        .clone()
        .into_iter()
        .filter(|rating| rating[column] == false)
        .collect();
    let ones: Vec<Vec<bool>> = ratings
        .clone()
        .into_iter()
        .filter(|rating| rating[column] == true)
        .collect();

    let remaining = if zeros.len() <= ratings.len() / 2 {
        zeros
    } else {
        ones
    };

    return get_co2_rating(remaining, column + 1);
}

fn get_decimal(binary: Vec<bool>) -> u32 {  
    if binary.len() == 1 {
        return binary[0] as u32;
    }

    return u32::pow(2, (binary.len() - 1) as u32) * (binary[0] as u32)
        + get_decimal(binary[1..].to_vec());
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut diagnostic = Diagnostic::new();

        for line in lines {
            if let Ok(line) = line {
                diagnostic.parse_line(line);
            }
        }

        let oxygen_rating = get_oxygen_rating(diagnostic.bits.clone(), 0);
        let co2_rating = get_co2_rating(diagnostic.bits.clone(), 0);

        let oxygen_rating = get_decimal(oxygen_rating[0].clone());
        let co2_rating = get_decimal(co2_rating[0].clone());

        println!("{}", oxygen_rating * co2_rating);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
