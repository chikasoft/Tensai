extern crate rand;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::RngCore;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    #[inline]
    pub const fn rotate(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Rotate,
    RotateIfNotZero,
    RotateIfZero,
    Increment,
    Decrement,
    Nop,
}

fn main() {
    let input_path = env::args()
        .skip(1)
        .next()
        .expect("No input file specified.");
    let input_file = File::open(input_path).expect("Failed to open input file.");

    let mut vertical = Vec::new();

    for line in BufReader::new(input_file).lines() {
        let mut horizontal = Vec::new();

        for c in line.expect("Failed to read input file.").chars() {
            match c {
                'R' => horizontal.push(Instruction::Rotate),
                '?' => horizontal.push(Instruction::RotateIfNotZero),
                '!' => horizontal.push(Instruction::RotateIfZero),
                '+' => horizontal.push(Instruction::Increment),
                '-' => horizontal.push(Instruction::Decrement),
                _ => {}
            }
        }

        vertical.push(horizontal);
    }

    let mut x = 0;
    let mut y = 0;
    let mut ip = Direction::East;
    let mut accumulator = rand::thread_rng().next_u64();

    loop {
        let instruction = if let Some(horizontal) = vertical.get(y) {
            if let Some(&inst) = horizontal.get(x) {
                inst
            } else {
                Instruction::Nop
            }
        } else {
            Instruction::Nop
        };

        match instruction {
            Instruction::Rotate => ip = ip.rotate(),
            Instruction::RotateIfNotZero => {
                if accumulator != 0 {
                    ip = ip.rotate();
                }
            }
            Instruction::RotateIfZero => {
                if accumulator == 0 {
                    ip = ip.rotate();
                }
            }
            Instruction::Increment => accumulator = accumulator.saturating_add(1),
            Instruction::Decrement => accumulator = accumulator.saturating_sub(1),
            Instruction::Nop => {}
        }

        match ip {
            Direction::North => y = y.saturating_sub(1),
            Direction::East => x = x.saturating_add(1),
            Direction::South => y = y.saturating_add(1),
            Direction::West => x = x.saturating_sub(1),
        }

        if x == 0 && y == 0 {
            break;
        }
    }
}
