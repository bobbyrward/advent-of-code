use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use regex::Regex;

use grid::{LightCommand, LightOp, Coord};


fn parse_line(line_re: &Regex, line: &str) -> LightCommand {
    let groups = match line_re.captures(&line) {
        Some(groups) => groups,
        None => panic!("Malformed line: {}", line),
    };

    let op = match groups.at(1).unwrap() {
        "turn on" => LightOp::TurnOn,
        "turn off" => LightOp::TurnOff,
        "toggle" => LightOp::Toggle,
        unknown => panic!("Unknown light operation: {}", unknown),
    };

    let start = Coord::new(
        groups.at(2).unwrap().parse::<u32>().unwrap(),
        groups.at(3).unwrap().parse::<u32>().unwrap(),
    );

    let stop = Coord::new(
        groups.at(4).unwrap().parse::<u32>().unwrap(),
        groups.at(5).unwrap().parse::<u32>().unwrap(),
    );

    LightCommand::new(op, start, stop)
}


pub fn read_commands(filename: &str) -> Vec<LightCommand> {
    let mut commands = Vec::new();
    let line_re = Regex::new(r"^(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    let data_file = match File::open(filename) {
        Err(reason) => panic!("Could not open data file {}: {}", filename, Error::description(&reason)),
        Ok(file) => file,
    };

    let reader = BufReader::new(data_file);

    for line in reader.lines() {
        let command_string = match line {
            Err(reason) => panic!("Could not read line: {}", Error::description(&reason)),
            Ok(data) => data,
        };

        commands.push(parse_line(&line_re, &command_string));
    }

    return commands;
}
