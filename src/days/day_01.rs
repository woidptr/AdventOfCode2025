use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

enum RotationDirection {
    Left,
    Right,
}

impl FromStr for RotationDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(RotationDirection::Left),
            "R" => Ok(RotationDirection::Right),
            other => Err(format!("Invalid direction: {}", other)),
        }
    }
}

pub fn compute() -> u32 {
    const DIAL_MAX: u8 = 100;

    let file = File::open("inputs/day_01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dial_position: u8 = 50;
    let mut password: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let index = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let (rotation, distance) = line.split_at(index);
        // println!("rotation: {}, distance: {}", rotation, distance);
        // let rotation: RotationDirection = rotation.parse().unwrap();
        let distance: u16 = distance.parse().unwrap();

        let m = DIAL_MAX as i32;
        if (rotation == "R") {
            dial_position = (((dial_position as i32) + distance as i32) % m) as u8;
        } else if (rotation == "L") {
            dial_position = (((dial_position as i32) - distance as i32 % m + m) % m) as u8;
        }

        if (dial_position == 0) {
            password += 1;
        }
    }

    password
}