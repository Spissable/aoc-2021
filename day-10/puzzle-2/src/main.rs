use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_valid_syntax(chars: Vec<char>) -> (bool, Option<usize>) {
    let brackets: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut stack: Vec<char> = vec![];
    let mut score = 0;

    for bracket in chars {
        if brackets.contains_key(&bracket) {
            stack.push(bracket);
            continue;
        }

        if stack.len() == 0 {
            return (false, None);
        }

        let checking = stack.pop().unwrap();

        if brackets[&checking] != bracket {
            return (false, None);
        }
    }

    while let Some(open_bracket) = stack.pop() {
        score *= 5;
        match open_bracket {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => {}
        };
    }

    return (true, Some(score));
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut scores: Vec<usize> = vec![];

        for line in lines {
            if let Ok(line) = line {
                let input: Vec<char> = line.chars().collect();
                match is_valid_syntax(input.clone()) {
                    (true, Some(score)) => {
                        scores.push(score);
                    }
                    _ => {}
                }
            }
        }
        scores.sort();
        println!("{:?}", scores.get(scores.len() / 2).unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
