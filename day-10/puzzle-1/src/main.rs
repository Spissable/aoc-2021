use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_valid_syntax(chars: Vec<char>) -> (bool, Option<char>) {
    let brackets: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut stack: Vec<char> = vec![];

    for bracket in chars {
        if brackets.contains_key(&bracket) {
            stack.push(bracket);
            continue;
        }

        if stack.len() == 0 {
            return (false, Some(bracket));
        }

        let checking = stack.pop().unwrap();

        if brackets[&checking] != bracket {
            return (false, Some(bracket));
        }
    }

    return (true, None);
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut result: usize = 0;

        for line in lines {
            if let Ok(line) = line {
                let input: Vec<char> = line.chars().collect();
                match is_valid_syntax(input) {
                    (false, Some(c)) => match c {
                        ')' => result += 3,
                        ']' => result += 57,
                        '}' => result += 1197,
                        '>' => result += 25137,
                        _ => {}
                    },
                    _ => {}
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
