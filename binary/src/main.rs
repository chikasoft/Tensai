use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() {
    let input_path = env::args()
        .skip(1)
        .next()
        .expect("No input file specified.");
    let input_file = File::open(input_path).expect("Failed to open input file.");

    let mut value = String::new();
    let mut instructions = Vec::new();

    for line in BufReader::new(input_file).lines() {
        for c in line.expect("Failed to read input file.").chars() {
            if c == '0' || c == '1' {
                value.push(c);

                if value.len() < 8 {
                    continue;
                }

                instructions.push(u8::from_str_radix(&value, 2).unwrap());
                value.clear();
            }
        }
    }

    if value.len() > 0 {
        println!("Incomplete binary value: {}", value);
        return;
    }

    let mut variables = HashMap::new();
    let mut instr_iter = instructions.into_iter();

    let mut stdin = io::stdin();

    loop {
        if let Some(instruction) = instr_iter.next() {
            match instruction {
                0 => {
                    let c = instr_iter
                        .next()
                        .expect("Tried to print a character but EOF was reached.");
                    print!("{}", c as char);
                    let _ = io::stdout().flush();
                }
                1 => {
                    let var = instr_iter
                        .next()
                        .expect("No variable specified to store input into.");
                    let mut input = [0];
                    stdin.read_exact(&mut input).expect("Failed to read input.");

                    variables
                        .entry(var)
                        .and_modify(|v| *v = input[0])
                        .or_insert(input[0]);
                }
                2 => {
                    let var = instr_iter
                        .next()
                        .expect("No variable specified to assign value.");
                    let value = instr_iter
                        .next()
                        .expect("No value specified to assign variable to.");

                    variables
                        .entry(var)
                        .and_modify(|v| *v = value)
                        .or_insert(value);
                }
                3 => {
                    let var = instr_iter
                        .next()
                        .expect("Tried to print a variable but EOF was reached.");
                    let value = variables.get(&var).map(|&v| v).unwrap_or_default();

                    print!("{}", value as char);
                    let _ = io::stdout().flush();
                }
                _ => {
                    println!("Unknown instruction: {:08b}", instruction);
                    return;
                }
            }
        } else {
            break;
        }
    }
}
