extern crate regex;

pub mod grid;
pub mod parser;

use grid::{LightGrid, LightCommand};
use parser::read_commands;


fn problem_one(data: &Vec<LightCommand>) {
    let mut grid = LightGrid::new(false, 1000, 1000);

    for command in data.iter() {
        grid.process_command(&command);
    }

    let enabled_count = grid.lights.iter().filter(|x| **x).count();
    println!("Problem One: {}", enabled_count);
}


fn problem_two(data: &Vec<LightCommand>) {
    let mut grid = LightGrid::new(0 as u8, 1000, 1000);

    for command in data.iter() {
        grid.process_command(&command);
    }

    let total_brightness = grid.lights.iter().fold(0 as u32, |accum, x| accum + *x as u32);
    println!("Problem Two: {}", total_brightness);
}


fn main() {
    let data = read_commands("../../data/day6.txt");
    problem_one(&data);
    problem_two(&data);
}
