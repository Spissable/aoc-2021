use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Dive {
    depth: u32,
    horizontal: u32,
}

impl Dive {
    fn new() -> Dive {
        return Dive {
            depth: 0,
            horizontal: 0,
        };
    }

    fn command(&mut self, cmd: String) {
        let tmp: Vec<&str> = cmd.split(" ").collect();
        let direction = tmp[0].parse::<String>().unwrap();
        let distance = tmp[1].parse::<u32>().unwrap();

        match direction.as_ref() {
            "forward" => self.horizontal += distance,
            "up" => self.depth -= distance,
            "down" => self.depth += distance,
            _ => {}
        };
    }

    fn get_result(&self) -> u32 {
        return self.depth * self.horizontal;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut dive = Dive::new();

        for line in lines {
            if let Ok(command) = line {
                dive.command(command);
            }
        }

        println!("{}", dive.get_result());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
