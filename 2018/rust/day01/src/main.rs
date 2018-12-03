use std::collections::HashSet;
use std::io::stdin;
use std::io::prelude::*;


fn part_one(deltas: &Vec<i32>) -> i32 {
    deltas.iter().sum()
}


fn part_two(deltas: &Vec<i32>) -> i32 {
    let mut frequencies = HashSet::new();

    // This should probably stop at cycle but...
    for freq in deltas.iter().cycle().scan(0, |acc, &x| { *acc += x; Some(*acc) }) {
        if !frequencies.insert(freq) {
            return freq;
        }
    }

    0
}


fn main() {
    let deltas: Vec<i32> = stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    println!("Part 1: {:?}", part_one(&deltas));
    println!("Part 2: {:?}", part_two(&deltas));
}
