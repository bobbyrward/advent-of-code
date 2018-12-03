use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;


const UP_CHAR: char = '^';
const DOWN_CHAR: char = 'v';
const LEFT_CHAR: char = '<';
const RIGHT_CHAR: char = '>';


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}


impl Position {
    fn move_char(&mut self, movement: char) {
        match movement {
            UP_CHAR => self.y += 1,
            DOWN_CHAR => self.y -= 1,
            LEFT_CHAR => self.x -= 1,
            RIGHT_CHAR => self.x += 1,
            _ => panic!("Unknown movement character"),
        }
    }
}


fn read_data() -> String {
    let file_path = "../../data/day3.txt";
    let mut data_file = match File::open(file_path) {
        Err(reason) => panic!("Could not open data file {}: {}", file_path, Error::description(&reason)),
        Ok(file) => file,
    };

    let mut data_buffer = String::new();
    match data_file.read_to_string(&mut data_buffer) {
        Err(reason) => panic!("Error reading file: {}", Error::description(&reason)),
        Ok(_) => {},
    }

    return data_buffer;
}


fn problem_one() {
    let data_buffer = read_data();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut position = Position{x: 0, y: 0};

    visited.insert(position);

    for movement in data_buffer.chars() {
        position.move_char(movement);
        visited.insert(position);
    }

    println!("Number visited: {}", visited.len());
}


fn problem_two() {
    let data_buffer = read_data();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut santa_position = Position{x: 0, y: 0};
    let mut robo_position = Position{x: 0, y: 0};

    visited.insert(santa_position);

    let mut buffer_iterator = data_buffer.chars();

    loop {
        let santa_move = match buffer_iterator.next() {
            Some(movement) => movement,
            None => break,
        };
        let robo_move = match buffer_iterator.next() {
            Some(movement) => movement,
            None => break,
        };
        santa_position.move_char(santa_move);
        visited.insert(santa_position);

        robo_position.move_char(robo_move);
        visited.insert(robo_position);
    }

    println!("Number visited: {}", visited.len());
}


fn main() {
    problem_one();
    problem_two();
}
