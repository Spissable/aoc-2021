use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CHECKS: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

fn is_lowpoint(grid: Vec<Vec<usize>>, x: usize, y: usize, index: usize) -> bool {
    if index == CHECKS.len().try_into().unwrap() {
        return true;
    }

    let (mut cmp_x, mut cmp_y) = CHECKS[index];
    cmp_x += x as isize;
    cmp_y += y as isize;

    if cmp_y >= 0 && cmp_x >= 0 {
        let cmp_x: usize = cmp_x.try_into().unwrap();
        let cmp_y: usize = cmp_y.try_into().unwrap();

        if cmp_y < grid.len() && cmp_x < grid[cmp_y].len() {
            if grid[y][x] >= grid[cmp_y][cmp_x] {
                return false;
            }
        }
    }

    return is_lowpoint(grid, x, y, index + 1);
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut grid: Vec<Vec<usize>> = vec![];

        for line in lines {
            if let Ok(line) = line {
                let numbers = line.chars().map(|char| char.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>();
                grid.push(numbers);
            }
        }

        let mut result: usize = 0;

        for (y, row) in grid.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if is_lowpoint(grid.clone(), x, y, 0) {
                    result += num + 1;
                }
            }
        }
        
        println!("{}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
