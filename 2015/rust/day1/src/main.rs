use std::error::Error;
use std::io::prelude::*;
use std::fs::File;


const UP_CHAR: char = '(';
const DOWN_CHAR: char = ')';


fn read_data() -> String {
    let file_path = "../../data/day1.txt";
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


fn main() {
    let data_buffer = read_data();
    let mut floor = 0;
    let mut current_move = 0;
    let mut has_entered_basement = false;

    for movement in data_buffer.chars() {
        current_move += 1;

        match movement {
            UP_CHAR => floor += 1,
            DOWN_CHAR => floor -= 1,
            _ => continue,
        }

        if !has_entered_basement && floor < 0 {
            println!("Entered basement at move {}", current_move);
            has_entered_basement = true;
        }
    }

    println!("Final floor: {}", floor);
}
