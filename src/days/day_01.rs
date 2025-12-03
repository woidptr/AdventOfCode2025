use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

pub struct Passwords {
    pub without_protocol: u32,
    pub with_protocol: u32,
}

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

pub fn compute() -> Passwords {
    const DIAL_MAX: u32 = 100;

    let file = File::open("inputs/day_01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dial_position: u32 = 50;
    let mut regular_password: u32 = 0;
    let mut protocol_password: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let index = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let (rotation, distance) = line.split_at(index);
        // println!("rotation: {}, distance: {}", rotation, distance);
        // let rotation: RotationDirection = rotation.parse().unwrap();
        let distance: u32 = distance.parse().unwrap();

        let old_position: u32 = dial_position;

        if (rotation == "R") {
            dial_position = (((dial_position) + distance) % DIAL_MAX);

            if (old_position < dial_position && distance >= DIAL_MAX) {
                protocol_password += distance / DIAL_MAX;
            }

            if (old_position + (distance % DIAL_MAX) >= DIAL_MAX) {
                protocol_password += 1;
            }
        } else if (rotation == "L") {
            dial_position = (dial_position + DIAL_MAX - (distance % DIAL_MAX)) % DIAL_MAX;

            if (distance >= DIAL_MAX) {
                protocol_password += distance / DIAL_MAX;
            }

            if ((distance % DIAL_MAX) >= old_position) {
                protocol_password += 1;
            }
        }

        if (dial_position == 0) {
            regular_password += 1;
        }
    }

    Passwords {
        without_protocol: regular_password,
        with_protocol: protocol_password,
    }
}