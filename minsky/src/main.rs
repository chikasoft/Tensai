use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Increment(usize),
    Decrement(usize, usize),
    Output(usize),
}

fn main() {
    let mut args = env::args().skip(1);
    let input_path = args.next().expect("No input file specified.");
    let use_chars = args.next().map_or(false, |s| s == "c");

    let input_file = File::open(input_path).expect("Failed to open input file.");
    let mut instructions = Vec::new();

    for line in BufReader::new(input_file).lines() {
        let line = line.expect("Failed to read input file.");

        if line.len() > 0 {
            let mut splitted = line.split_whitespace();

            let instruction = splitted.next().unwrap().to_lowercase();
            let register = splitted
                .next()
                .expect("No register specified.")
                .parse::<usize>()
                .expect("Invalid register.");

            match instruction.as_str() {
                "inc" => instructions.push(Instruction::Increment(register)),
                "dec" => {
                    let jump_location = splitted
                        .next()
                        .expect("No jump location specified.")
                        .parse::<usize>()
                        .expect("Invalid jump location.");

                    instructions.push(Instruction::Decrement(register, jump_location));
                }
                "out" => instructions.push(Instruction::Output(register)),
                _ => println!("Unknown instruction: {}", line),
            }
        }
    }

    let mut location = 0;
    let mut registers: HashMap<usize, usize> = HashMap::new();

    while location < instructions.len() {
        let instruction = unsafe { instructions.get_unchecked(location) };

        match instruction {
            Instruction::Increment(register) => {
                let register = registers.entry(*register).or_default();
                *register = register.saturating_add(1);
            }
            Instruction::Decrement(register, jump_location) => {
                let register = registers.entry(*register).or_default();

                if *register > 0 {
                    *register = register.saturating_sub(1);
                } else {
                    location = *jump_location;
                }
            }
            Instruction::Output(register) => {
                let value = *registers.entry(*register).or_default();

                if use_chars {
                    if value <= 255 {
                        print!("{}", value as u8 as char);
                    } else {
                        print!("{}", value);
                    }
                } else {
                    println!("{}", value);
                }
            }
        }

        location += 1;
    }
}
