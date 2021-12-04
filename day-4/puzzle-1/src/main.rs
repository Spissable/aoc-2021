use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct Board {
    numbers: Vec<Vec<(u32, bool)>>,
}

impl Board {
    fn new() -> Board {
        return Board { numbers: vec![] };
    }

    fn check_number(&mut self, number: u32) -> bool {
        for (row_index, row) in self.numbers.iter_mut().enumerate() {
            for (column_index, (board_number, checked)) in row.iter_mut().enumerate() {
                if *board_number == number {
                    *checked = true;
                    return self.has_won(row_index, column_index);
                }
            }
        }

        return false;
    }

    fn has_won(&self, row_index: usize, column_index: usize) -> bool {
        let row = &self.numbers[row_index as usize];
        if row
            .iter()
            .filter(|(_, checked)| *checked == false)
            .collect::<Vec<&(u32, bool)>>()
            .len()
            == 0
        {
            return true;
        }
        let column = &self
            .numbers
            .iter()
            .map(|row| row[column_index])
            .collect::<Vec<(u32, bool)>>();
        if column
            .iter()
            .filter(|(_, checked)| *checked == false)
            .collect::<Vec<&(u32, bool)>>()
            .len()
            == 0
        {
            return true;
        }
        return false;
    }

    fn get_score(&self, last_num: u32) -> u32 {
        let mut score: u32 = 0;
        for row in self.numbers.iter() {
            for (board_number, checked) in row.iter() {
                if *checked == false {
                    score += *board_number;
                }
            }
        }
        return score * last_num;
    }
}

struct Game {
    current_index: u32,
    draw: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn new(draw: Vec<u32>, boards: Vec<Board>) -> Game {
        return Game {
            current_index: 0,
            draw,
            boards,
        };
    }

    fn draw_numbers(&mut self) -> u32 {
        for number in self.draw.iter() {
            for board in self.boards.iter_mut() {
                if board.check_number(*number) == true {
                    return board.get_score(*number);
                }
            }
        }

        self.current_index += 1;
        return 0;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut draw: Vec<u32> = vec![];
        let mut all_boards: Vec<Board> = vec![];
        let mut tmp_board: Board = Board::new();

        for (index, line) in lines.enumerate() {
            if let Ok(line) = line {
                if index == 0 {
                    let split: Vec<&str> = line.split(",").collect();
                    draw = split
                        .iter()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect();
                } else if !line.is_empty() {
                    let split: Vec<&str> = line.split_whitespace().collect();
                    let numbers: Vec<(u32, bool)> = split
                        .iter()
                        .map(|num| (num.parse::<u32>().unwrap(), false))
                        .collect();
                    tmp_board.numbers.push(numbers);

                    if tmp_board.numbers.len() == 5 {
                        all_boards.push(tmp_board.clone());
                        tmp_board = Board::new();
                    }
                }
            }
        }
        println!("{:#?}", all_boards);

        let mut game = Game::new(draw, all_boards);
        let score = game.draw_numbers();
        println!("{}", score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
