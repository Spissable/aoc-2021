use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Lanternfish {
    ctr: usize,
}

impl Lanternfish {
    fn new(ctr: Option<usize>) -> Lanternfish {
        match ctr {
            Some(ctr) => return Lanternfish {ctr},
            None => return Lanternfish {ctr: 8},
        }
    }

    fn pass_day(&mut self) -> Option<Lanternfish> {
        if self.ctr == 0 {
            self.ctr = 6;
            return Some(Lanternfish::new(None));
        }

        self.ctr -= 1;
        return None;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lanternfishes: Vec<Lanternfish> = vec![];

        for line in lines {
            if let Ok(line) = line {
                line.split(",").for_each(|t| lanternfishes.push(Lanternfish::new(Some(t.parse::<usize>().unwrap()))));
            }
        }
        
        for _ in 0..80 {
            let mut new_fishes: Vec<Lanternfish> = vec![];
            lanternfishes.iter_mut().for_each(|fish| {
                let fish = fish.pass_day();
                match fish {
                    Some(new_fish) => new_fishes.push(new_fish),
                    None => {},
                }
            });

            lanternfishes.extend(new_fishes);
        }

        println!("{:#?}", lanternfishes.len());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
