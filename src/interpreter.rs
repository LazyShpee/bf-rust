use std::fs;
use std::cmp;
use std::io::{stdin, Read, stdout, Write};

use colored::*;

pub struct Options {
    pub verbose: bool,
    pub tape_dump: bool,
    pub color: bool,
    pub tape_length: usize,
    pub filename: Option<String>,
    pub code: Option<String>,
    pub extend: u8,
}

#[derive(Default)]
pub struct VM {
    program: String,
    tape: Vec<u8>,

    data_ptr: usize,
    code_ptr: usize,

    data_start: usize,
    code_start: usize,

    running: bool,
    extend: u8,
}

enum ChangeMode<T> {
    Add(T),
    Sub(T),
    Set(T),
}

impl VM {
    pub fn new(options: &mut Options) -> VM {
        let mut vm = VM {
            program: if let Option::Some(filename) = &options.filename {
                if options.verbose { println!("Reading program from {}", filename) };
                fs::read_to_string(filename).expect("Something went wrong reading the file")
            } else if let Option::Some(ref code) = options.code {
                if options.verbose { println!("Reading program from command line") };
                code.clone()
            } else {
                if options.verbose { println!("Reading program from stdin") };
                let mut code: String = String::new();
                stdin().read_to_string(&mut code).expect("Could not read code from stdin");
                code
            },

            running: true,
            extend: options.extend,

            ..Default::default()
        };
        vm.tape.push(0); // Storage

        vm.code_start = vm.tape.len(); // Actual program
        vm.code_ptr = vm.code_start;
        vm.tape.extend(vm.program.as_bytes());

        vm.data_start = vm.tape.len(); // Data
        vm.data_ptr = vm.data_start;
        vm.tape.extend(vec![0u8; options.tape_length]);

        if options.verbose { println!("Created VM type {} with code {}", vm.extend, String::from_utf8(vm.tape[vm.code_start..vm.data_start - 1].to_vec()).unwrap_or("INVALID CODE".to_string())) }

        vm
    }

    fn tape_change(&mut self, mode: ChangeMode<u8>) {
        match mode {
            ChangeMode::Add(value) => self.tape[self.data_ptr] = self.tape[self.data_ptr].wrapping_add(value),
            ChangeMode::Sub(value) => self.tape[self.data_ptr] = self.tape[self.data_ptr].wrapping_sub(value),
            ChangeMode::Set(value) => self.tape[self.data_ptr] = value,
        }
    }

    fn data_ptr_change(&mut self, mode: ChangeMode<usize>) {
        // Tape is writable from lower to upper, will change with --x2 and --x3
        let lower = self.data_start;
        let upper = self.tape.len() - 1;

        match mode {
            ChangeMode::Add(value) => {
                if self.data_ptr + value <= upper {
                    self.data_ptr += value;
                } else {
                    self.data_ptr = lower;
                }
            },
            ChangeMode::Sub(value) => {
                if lower <= self.data_ptr - value {
                    self.data_ptr -= value;
                } else {
                    self.data_ptr = upper;
                }
            },
            ChangeMode::Set(value) => if lower <= value && value <= upper { self.data_ptr = value },
        }
    }

    pub fn run(&mut self) {
        while self.running && self.code_ptr < self.tape.len() {
            match self.tape[self.code_ptr] as char {
                '+' => self.tape_change(ChangeMode::Add(1)),
                '-' => self.tape_change(ChangeMode::Sub(1)),
                '>' => self.data_ptr_change(ChangeMode::Add(1)),
                '<' => self.data_ptr_change(ChangeMode::Sub(1)),
                '.' => { stdout().write(&[self.tape[self.data_ptr]]).expect("Could not write to stdout"); () },
                ',' => {
                    let mut buffer: [u8; 1] = [0; 1];

                    stdin().read(&mut buffer[..]).expect("Could not read from stdin");
                    self.tape_change(ChangeMode::Set(if buffer[0] == b'\n' { 0 } else { buffer[0] }));
                },
                '[' => {
                    if self.tape[self.data_ptr] == 0 {
                        let mut count: usize = 1;
                        while count > 0 {
                            self.code_ptr += 1;
                            match self.tape[self.code_ptr] as char {
                                '[' => count += 1,
                                ']' => count -= 1,
                                _ => (),
                            }
                        }
                    }
                },
                ']' => {
                    if self.tape[self.data_ptr] != 0 {
                        let mut count: usize = 1;
                        while count > 0 {
                            self.code_ptr -= 1;
                            match self.tape[self.code_ptr] as char {
                                '[' => count -= 1,
                                ']' => count += 1,
                                _ => (),
                            }
                        }
                    }
                },
                _ => (),
            }

            if self.extend >= 1 {
                match self.tape[self.code_ptr] as char {
                    '@' => self.running = false,
                    '$' => self.tape[0] = self.tape[self.data_ptr],
                    '!' => { let n = self.tape[0]; self.tape_change(ChangeMode::Set(n)) },
                    '{' => { let n = self.tape[self.data_ptr] << 1; self.tape_change(ChangeMode::Set(n)) },
                    '}' => { let n = self.tape[self.data_ptr] >> 1; self.tape_change(ChangeMode::Set(n)) },
                    '~' => { let n = !self.tape[self.data_ptr]; self.tape_change(ChangeMode::Set(n)) },
                    '^' => { let n = self.tape[self.data_ptr] ^ self.tape[0]; self.tape_change(ChangeMode::Set(n)) },
                    '&' => { let n = self.tape[self.data_ptr] & self.tape[0]; self.tape_change(ChangeMode::Set(n)) },
                    '|' => { let n = self.tape[self.data_ptr] | self.tape[0]; self.tape_change(ChangeMode::Set(n)) },
                    _ => (),
                }
            } else {
                self.running = self.code_ptr < self.data_start - 1;
            }

            self.code_ptr += 1;
        }
    }

    /**
     * This just dumps the current memory (storage, code and memory tape)
     */
    pub fn dump(&self, colors: bool) {
        for ptr in (0..self.tape.len() - 1).step_by(16) {
            let to = cmp::min(ptr + 16, self.tape.len() - 1);
            print!("{:08X} ", ptr);

            // Printing hex values
            for index in ptr..to {
                let s = format!("{:02X}", self.tape[index]);

                let mut s2 = if index >= self.data_start {
                    s.green()
                } else if index >= self.code_start {
                    s.bright_blue()
                } else {
                    s.red()
                };

                if index == self.data_ptr {
                    s2 = s2.underline()
                }

                print!(" {}", if colors { s2 } else { s.normal() });
            }

            // Padding if last line of hex representation isn't 16 bytes
            for _ in 1..17 - (to - ptr) {
                print!(" 00");
            }
            
            // Printing ascii representation
            print!("  |");
            for index in ptr..to {
                let c = self.tape[index];

                let s = format!("{}", if 32 <= c && c <= 126 { c as char } else { '.' } );
                
                let mut s2 = if index >= self.data_start {
                    s.green()
                } else if index >= self.code_start {
                    s.bright_blue()
                } else {
                    s.red()
                };

                if index == self.data_ptr {
                    s2 = s2.underline()
                }

                print!("{}", if colors { s2 } else { s.normal() });
            }

            // Padding if last line of ascii representation isn't 16 bytes
            for _ in 1..17 - (to - ptr) {
                print!(".");
            }

            print!("|");

            println!();
        }
    }
}