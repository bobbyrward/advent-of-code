#![feature(slice_patterns, convert)]

extern crate permutohedron;

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use permutohedron::Heap;


fn main() {
    let data = include_str!("../../../data/day9.txt");
    let mut distances: HashMap<(&str, &str), u32> = HashMap::new();
    let mut locations: HashSet<&str> = HashSet::new();

    for current_line in data.lines() {
        let tokens: Vec<&str> = current_line.split(" ").collect();

        match &tokens[..] {
            [from, "to", to, "=", distance] => {
                distances.insert((from, to), distance.parse::<u32>().unwrap());
                distances.insert((to, from), distance.parse::<u32>().unwrap());
                locations.insert(from);
                locations.insert(to);
            },
            _ => panic!("unrecognized line"),
        };
    }

    let mut keys: Vec<&str> = locations.iter().map(|x| *x).collect();
    let mut permutations = Heap::new(keys.as_mut_slice());
    let mut min_distance: u32 = !0;
    let mut max_distance: u32 = 0;

    loop {
        let route = match permutations.next_permutation() {
            Some(route) => route,
            None => break,
        };

        let distance: u32 = route.windows(2).map(|segment| *distances.get(&(segment[0], segment[1])).unwrap()).fold(0, |acc, x| acc + x);

        min_distance = cmp::min(min_distance, distance);
        max_distance = cmp::max(max_distance, distance);
    }

    println!("Min distance: {}", min_distance);
    println!("Max distance: {}", max_distance);
}
