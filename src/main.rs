use std::fs;
use std::io;
use std::env::args;
use std::io::Write;

fn main() {
    let mut machine = TapeMachine {
        tape: vec![0; 30000],
        data_ptr: 0,
    };
    if args().len() < 2 {
        // Running with interactive mode
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "quit"{
                break;
            }
            execute(&input, &mut machine)
        }
    }else{
        // Running with file mode
        let filename = args().nth(1).expect("No filename provided");
        execute(&parse_file(&filename), &mut machine);
        debug_print(&machine);
    }
}

struct TapeMachine{
    tape: Vec<u8>,
    data_ptr: usize,
}

fn parse_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Unable to read file");
    return contents;
}

fn debug_print(machine: &TapeMachine) {
    log::debug!("{:?}", &machine.tape[0..10]);
}

fn execute(cmd: &str, machine: &mut TapeMachine) {
    let mut i = 0;
    while i < cmd.len() {
        let c = cmd.chars().nth(i).unwrap();
        match c {
            '+' => {
                machine.tape[machine.data_ptr] += 1;
            }
            '-' => {
                machine.tape[machine.data_ptr] -= 1;
            }
            '>' => {
                if machine.tape.len() <= machine.data_ptr {
                    machine.tape.push(0);
                }
                machine.data_ptr += 1;
            }
            '<' => {
                if machine.data_ptr == 0 {
                    panic!("Moving pointer to negative index!");
                }
                machine.data_ptr -= 1;
            }
            '.' => {
                print!("{}", &(machine.tape[machine.data_ptr] as char));
                io::stdout().flush().unwrap();
            }
            ',' => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                machine.tape[machine.data_ptr] = match input.trim().chars().nth(0) {
                    Some(c) => c as u8,
                    None => 10,
                    };
            }
            '[' => {
                if machine.tape[machine.data_ptr] == 0 {
                    let mut bracket_count = 0;
                    for j in i+1..cmd.len() {
                        let character = cmd.chars().nth(j).unwrap();
                        if character == '[' {
                            bracket_count += 1;
                        }
                        if character == ']' && bracket_count == 0 {
                            i = j;
                            break;
                        }
                        if character == ']' {
                            bracket_count -= 1;
                        }
                    }
                    if bracket_count != 0 {
                        panic!("Unmatched brackets [");
                    }
                }
            }
            ']' => {
                if machine.tape[machine.data_ptr] != 0 {
                    let mut bracket_count = 0;
                    for j in (0..i).rev() {
                        let character = cmd.chars().nth(j).unwrap();
                        if character == ']' {
                            bracket_count += 1;
                        }
                        if character == '[' && bracket_count == 0 {
                            i = j;
                            break;
                        }
                        if character == '[' {
                            bracket_count -= 1;
                        }
                    }
                    if bracket_count != 0 {
                        panic!("Unmatched brackets ]");
                    }
                }
            }
            _ => {
            }
        }
        i += 1;
    }
}