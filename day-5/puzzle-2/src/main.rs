use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct VentMap {
    vents: [[u32; 1000]; 1000],
}

impl VentMap {
    fn new() -> VentMap {
        return  VentMap { vents: [[0; 1000]; 1000]};
    }

    fn add_line(& mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.vents[x1][y1] += 1;

        if x1 == x2 && y1 == y2 {
            return;
        }

        let new_x1 = if x1 == x2 {x1} else if x1 < x2 {x1+1} else {x1-1};
        let new_y1 = if y1 == y2 {y1} else if y1 < y2 {y1+1} else {y1-1};

        return self.add_line(new_x1, new_y1, x2, y2)
    }

    fn count_overlaps(&self, amount: u32) -> usize {
        let mut result: usize = 0;
        for row in self.vents.iter() {
            for val in row.iter() {
                if *val >= amount {
                    result += 1;
                }
            }
        }
        return result;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut map = VentMap::new();

        for line in lines {
            if let Ok(line) = line {
                let coord_split: Vec<&str> = line.split(" -> ").collect();
                let start_coords: Vec<usize> = coord_split[0].split(",").map(|coord| coord.parse::<usize>().unwrap()).collect();
                let end_coords: Vec<usize> = coord_split[1].split(",").map(|coord| coord.parse::<usize>().unwrap()).collect();
                
                map.add_line(start_coords[0], start_coords[1], end_coords[0], end_coords[1]);
            }
        }

        println!("{:#?}", map.count_overlaps(2));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
