extern crate regex;

pub mod wiring;


use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use wiring::{Circuit, NodeReference, Operation};


fn read_data() -> Vec<String> {
    let file_path = "../../data/day7.txt";
    let data_file = match File::open(file_path) {
        Err(reason) => panic!("Could not open data file {}: {}", file_path, Error::description(&reason)),
        Ok(file) => file,
    };

    let reader = BufReader::new(data_file);
    reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>()
}


fn load_circuit() -> wiring::Circuit {
    let mut circuit = wiring::Circuit::new();
    let data = read_data();

    for line in data.iter() {
        let parts: Vec<&str> = line.split(" ").collect();
        let num_parts = parts.len();

        let output = &parts[num_parts-1];

        match parts.len() {
            3 => match parts[0].parse::<u16>() {
                Ok(literal) => circuit.add_connection(output, Operation::DIRECT, NodeReference::Literal(literal), None),
                Err(_) => circuit.add_connection(output, Operation::DIRECT, NodeReference::Named(parts[0].to_string()), None),
            },
            4 => {
                let lhs = match parts[1].parse::<u16>() {
                    Ok(literal) => NodeReference::Literal(literal),
                    Err(_) => NodeReference::Named(parts[1].to_string()),
                };

                circuit.add_connection(output, Operation::NOT, lhs, None)
            }
            5 => {
                let lhs = match parts[0].parse::<u16>() {
                    Ok(literal) => NodeReference::Literal(literal),
                    Err(_) => NodeReference::Named(parts[0].to_string()),
                };

                let rhs = match parts[2].parse::<u16>() {
                    Ok(literal) => NodeReference::Literal(literal),
                    Err(_) => NodeReference::Named(parts[2].to_string()),
                };

                let op = match parts[1] {
                    "AND" => Operation::AND,
                    "OR" => Operation::OR,
                    "LSHIFT" => Operation::LSHIFT,
                    "RSHIFT" => Operation::RSHIFT,
                    unknown => panic!("Unknown operation {}", unknown),
                };

                circuit.add_connection(output, op, lhs, Some(rhs));
            },
            _ => panic!("Unknown line format: {}", line),
        }
    }

    circuit
}

fn test_one() {
    let mut circuit = load_circuit();
    println!("Problem One: {}", circuit.resolve_reference(&NodeReference::Named("a".to_string())));
}

fn test_two() {
    let mut circuit = load_circuit();
    circuit.add_connection("b", Operation::DIRECT, NodeReference::Literal(3176), None);
    println!("Problem One: {}", circuit.resolve_reference(&NodeReference::Named("a".to_string())));
}


fn main() {
    test_one();
    test_two();
}

#[test]
fn test_circuit() {
    let mut circuit = wiring::Circuit::new();

    circuit.add_connection("x", Operation::DIRECT, NodeReference::Literal(123), None);
    circuit.add_connection("y", Operation::DIRECT, NodeReference::Literal(456), None);
    circuit.add_connection("d", Operation::AND, NodeReference::Named("x".to_string()), Some(NodeReference::Named("y".to_string())));
    circuit.add_connection("e", Operation::OR, NodeReference::Named("x".to_string()), Some(NodeReference::Named("y".to_string())));
    circuit.add_connection("f", Operation::LSHIFT, NodeReference::Named("x".to_string()), Some(NodeReference::Literal(2)));
    circuit.add_connection("g", Operation::RSHIFT, NodeReference::Named("y".to_string()), Some(NodeReference::Literal(2)));
    circuit.add_connection("h", Operation::NOT, NodeReference::Named("x".to_string()), None);
    circuit.add_connection("i", Operation::NOT, NodeReference::Named("y".to_string()), None);

    assert_eq!(72, circuit.resolve_reference(&NodeReference::Named("d".to_string())));
    assert_eq!(507, circuit.resolve_reference(&NodeReference::Named("e".to_string())));
    assert_eq!(492, circuit.resolve_reference(&NodeReference::Named("f".to_string())));
    assert_eq!(114, circuit.resolve_reference(&NodeReference::Named("g".to_string())));
    assert_eq!(65412, circuit.resolve_reference(&NodeReference::Named("h".to_string())));
    assert_eq!(65079, circuit.resolve_reference(&NodeReference::Named("i".to_string())));
    assert_eq!(123, circuit.resolve_reference(&NodeReference::Named("x".to_string())));
    assert_eq!(456, circuit.resolve_reference(&NodeReference::Named("y".to_string())));

}
