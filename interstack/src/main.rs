use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::num::Wrapping;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Push,
    Pop,
    Peek,
    Swap,
    SetValue,
    Flip,
    Set0,
    Set65,
    Input,
    Output,
    End,
    Decrement,
    Increment,
    AddTop,
    OpenLoop,
    CloseLoop(usize),
    BreakLoop(usize),
}

struct Loop {
    start: usize,
    breaks: Vec<usize>,
}

impl Loop {
    #[inline]
    pub const fn new(start: usize) -> Self {
        Loop {
            start,
            breaks: Vec::new(),
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let input_path = args.next().expect("No input file specified.");
    let use_chars = args.next().map_or(false, |s| s == "c");

    let input_file = File::open(input_path).expect("Failed to open input file.");

    let mut loops = Vec::new();
    let mut instructions = Vec::new();

    for line in BufReader::new(input_file).lines() {
        let line = line.expect("Failed to read from input file.");

        for c in line.chars() {
            match c {
                '+' => instructions.push(Instruction::Push),
                '^' => instructions.push(Instruction::Pop),
                '@' => instructions.push(Instruction::Peek),
                '%' => instructions.push(Instruction::Swap),
                '_' => instructions.push(Instruction::SetValue),
                '~' => instructions.push(Instruction::Flip),
                '*' => instructions.push(Instruction::Set0),
                '#' => instructions.push(Instruction::Set65),
                '?' => instructions.push(Instruction::Input),
                '!' => instructions.push(Instruction::Output),
                '.' => instructions.push(Instruction::End),
                '<' => instructions.push(Instruction::Decrement),
                '>' => instructions.push(Instruction::Increment),
                '&' => instructions.push(Instruction::AddTop),
                '(' => {
                    loops.push(Loop::new(instructions.len()));
                    instructions.push(Instruction::OpenLoop);
                }
                ')' => {
                    if let Some(l) = loops.pop() {
                        let end_position = instructions.len();

                        for b in l.breaks {
                            if let Some(Instruction::BreakLoop(end)) = instructions.get_mut(b) {
                                *end = end_position;
                            }
                        }

                        instructions.push(Instruction::CloseLoop(l.start));
                    }
                }
                ';' => {
                    if let Some(l) = loops.last_mut() {
                        l.breaks.push(instructions.len());
                        instructions.push(Instruction::BreakLoop(0));
                    }
                }
                _ => {}
            }
        }
    }

    let mut position = 0;
    let mut value: u8 = 0;

    let mut stack: Vec<u8> = Vec::new();
    let mut loops: Vec<u8> = Vec::new();

    let stdin = io::stdin();

    while position < instructions.len() {
        let instruction = unsafe { instructions.get_unchecked(position) };

        match instruction {
            Instruction::Push => {
                stack.push(value);
                value = 0;
            }
            Instruction::Pop => {
                value = stack.pop().unwrap_or_else(|| {
                    println!("Tried to pop while stack is empty - returning 0.");
                    0
                });
            }
            Instruction::Peek => {
                value = stack.last().map_or_else(
                    || {
                        println!("Tried to peek while stack is empty - returning 0.");
                        0
                    },
                    |&v| v,
                );
            }
            Instruction::Swap => {
                let new_value = stack.pop().unwrap_or_else(|| {
                    println!("Tried to pop while stack is empty - returning 0.");
                    0
                });

                stack.push(value);
                value = new_value;
            }
            Instruction::SetValue => {
                let _ = stack.pop();
                stack.push(value);
                value = 0;
            }
            Instruction::Flip => {
                stack.reverse();
            }
            Instruction::Set0 => {
                value = 0;
            }
            Instruction::Set65 => {
                value = 65;
            }
            Instruction::Input => {
                let mut input = String::new();
                stdin
                    .read_line(&mut input)
                    .expect("Failed to read user input.");

                value = input
                    .chars()
                    .filter_map(|c| {
                        if c != '\n' && c != '\r' {
                            Some(Wrapping(c as u8))
                        } else {
                            None
                        }
                    })
                    .sum::<Wrapping<u8>>()
                    .0;
            }
            Instruction::Output => {
                if use_chars {
                    print!("{}", value as char);
                    let _ = io::stdout().flush();
                } else {
                    print!("{}", value);
                    let _ = io::stdout().flush();
                }
            }
            Instruction::End => {
                return;
            }
            Instruction::Decrement => {
                value = value.wrapping_sub(1);
            }
            Instruction::Increment => {
                value = value.wrapping_add(1);
            }
            Instruction::AddTop => {
                let top = stack.pop().unwrap_or_default();
                stack.push(top + value);
                value = 0;
            }
            Instruction::OpenLoop => {
                loops.push(value);
            }
            Instruction::CloseLoop(start_position) => {
                if let Some(iteration) = loops.pop() {
                    if iteration > 0 {
                        loops.push(iteration - 1);
                        position = *start_position;
                    }
                }
            }
            Instruction::BreakLoop(end_position) => {
                let _ = loops.pop();
                position = *end_position;
            }
        }

        position += 1;
    }
}
