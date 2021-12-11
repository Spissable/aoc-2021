use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CHECKS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn step(mut grid: Vec<Vec<usize>>) {
    let mut all_blinking = false;
    let mut step = 0;
    while all_blinking == false {
        all_blinking = true;
        step += 1;

        let mut blinking: Vec<(usize, usize)> = vec![];

        for (row_i, row) in grid.iter_mut().enumerate() {
            for (col_i, num) in row.iter_mut().enumerate() {
                if *num == 9 {
                    *num = 0;
                    blinking.push((row_i, col_i));
                } else {
                    *num += 1;
                }
            }
        }

        while blinking.len() > 0 {
            let (row, col) = blinking.pop().unwrap();

            for (check_row, check_col) in CHECKS {
                let check_row = check_row + row as isize;
                let check_col = check_col + col as isize;

                if check_row >= 0 && check_col >= 0 {
                    let check_row: usize = check_row.try_into().unwrap();
                    let check_col: usize = check_col.try_into().unwrap();

                    if check_row < grid.len() && check_col < grid[check_row].len() {
                        if grid[check_row][check_col] == 9 {
                            grid[check_row][check_col] = 0;
                            blinking.push((check_row, check_col));
                        } else if grid[check_row][check_col] != 0 {
                            grid[check_row][check_col] += 1;
                        }
                    }
                }
            }
        }

        for row in grid.iter() {
            for num in row.iter() {
                if *num != 0 {
                    all_blinking = false;
                    break;
                }
            }
        }

        if all_blinking == true {
            println!("{}", step);
            break;
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut grid: Vec<Vec<usize>> = vec![];

        for line in lines {
            if let Ok(line) = line {
                let row: Vec<usize> = line
                    .chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect();
                grid.push(row);
            }
        }

        step(grid);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
