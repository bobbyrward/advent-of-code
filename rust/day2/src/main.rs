use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;


struct Box {
    length: i32,
    width: i32,
    height: i32,
}


impl Box {
    fn surface_area(&self) -> i32 {
        return 2 * self.length * self.width +
               2 * self.width * self.height +
               2 * self.height * self.length;
    }

    fn slack_needed(&self) -> i32 {
        let (s1, s2)  = self.smallest_side();
        return s1 * s2;
    }

    fn shortest_distance(&self) -> i32 {
        let (s1, s2)  = self.smallest_side();
        return s1 * 2 + s2 * 2;
    }

    fn smallest_side(&self) -> (i32, i32) {
        if self.length <= self.width {
            if self.width <= self.height {
                return (self.length, self.width);
            } else {
                if self.length <= self.height {
                    return (self.length, self.height);
                } else {
                    return (self.height, self.length);
                }
            }
        } else {
            if self.length <= self.height {
                return (self.width, self.length);
            } else {
                if self.width < self.height {
                    return (self.width, self.height);
                } else {
                    return (self.height, self.width);
                }
            }
        }
    }

    fn volume(&self) -> i32 {
        return self.length * self.width * self.height
    }

    fn required_paper(&self) -> i32 {
        return self.surface_area() + self.slack_needed();
    }

    fn required_ribbon(&self) -> i32 {
        return self.shortest_distance() + self.volume();
    }
}


fn read_data() -> Vec<String> {
    let file_path = "../../data/day2.txt";
    let data_file = match File::open(file_path) {
        Err(reason) => panic!("Could not open data file {}: {}", file_path, Error::description(&reason)),
        Ok(file) => file,
    };

    let reader = BufReader::new(data_file);
    reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>()
}


fn parse_line(line: &String) -> Box {
    let mut parsed = Box {
        length: 0,
        height: 0,
        width: 0,
    };

    for (index, part) in line.split("x").enumerate() {
        let parsed_number = match part.parse::<i32>() {
            Ok(number) => number,
            Err(reason) => panic!("Error parsing dimensions: {}", reason),
        };

        match index {
            0 => parsed.length = parsed_number,
            1 => parsed.width = parsed_number,
            2 => parsed.height = parsed_number,
            _ => panic!("Box has more than 3 dimensions"),
        };
    }

    return parsed;
}


fn main() {
    let data_lines = read_data();
    let mut boxes: Vec<Box> = Vec::new();

    for line in data_lines.iter() {
        boxes.push(parse_line(line));
    }

    println!(
        "Required paper: {}", 
        boxes.iter().fold(0, |acc, b| acc + b.required_paper()),
    );
    println!(
        "Required ribbon: {}", 
        boxes.iter().fold(0, |acc, b| acc + b.required_ribbon()),
    );
}


#[test]
fn test_part1_example1() {
    let test_box = Box{length: 2,  width: 3, height: 4};
    assert_eq!((2,3), test_box.smallest_side());
    assert_eq!(52, test_box.surface_area());
    assert_eq!(6, test_box.slack_needed());
    assert_eq!(58, test_box.required_paper());
}


#[test]
fn test_part1_example2() {
    let test_box = Box{length: 1,  width: 1, height: 10};
    assert_eq!((1,1), test_box.smallest_side());
    assert_eq!(42, test_box.surface_area());
    assert_eq!(1, test_box.slack_needed());
    assert_eq!(43, test_box.required_paper());
}


#[test]
fn test_part2_example1() {
    let test_box = Box{length: 2,  width: 3, height: 4};
    assert_eq!((2,3), test_box.smallest_side());
    assert_eq!(10, test_box.shortest_distance());
    assert_eq!(24, test_box.volume());
    assert_eq!(34, test_box.required_ribbon());
}


#[test]
fn test_part2_example2() {
    let test_box = Box{length: 1,  width: 1, height: 10};
    assert_eq!((1,1), test_box.smallest_side());
    assert_eq!(4, test_box.shortest_distance());
    assert_eq!(10, test_box.volume());
    assert_eq!(14, test_box.required_ribbon());
}


#[test]
fn test_smallest_side() {
    let lw = Box {length: 1, height: 2, width: 3};
    assert_eq!((1,2), lw.smallest_side());

    let lw = Box {length: 4, height: 2, width: 3};
    assert_eq!((2,3), lw.smallest_side());

    let lw = Box {length: 3, height: 5, width: 4};
    assert_eq!((3,4), lw.smallest_side());

    let lw = Box {length: 6, height: 5, width: 4};
    assert_eq!((4,5), lw.smallest_side());

    let lw = Box {length: 3, height: 1, width: 1};
    assert_eq!((1,1), lw.smallest_side());

    let lw = Box {length: 1, height: 3, width: 1};
    assert_eq!((1,1), lw.smallest_side());

    let lw = Box {length: 1, height: 1, width: 3};
    assert_eq!((1,1), lw.smallest_side());
}
