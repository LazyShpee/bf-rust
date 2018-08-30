extern crate argparse;

use std::fs;
use std::io::{stdin, Read, stdout, Write};

use argparse::{ArgumentParser, StoreTrue, StoreOption};

/**
 * This function assumes &program is CLEAN,
 * meaning every [ has a matching ] and no [] is present
 */
fn execute(program: &[u8], tape: &mut Vec<u8>) -> usize {
    let mut tape_ptr: usize = 0;
    let mut inst_ptr: usize = 0;

    while inst_ptr < program.len() {
        match program[inst_ptr] {
            b'+' => tape[tape_ptr] = tape[tape_ptr].wrapping_add(1),
            b'-' => tape[tape_ptr] = tape[tape_ptr].wrapping_sub(1),
            b'>' => tape_ptr = if tape_ptr + 1 < tape.len() { tape_ptr + 1 } else { tape.len() - 1 },
            b'<' => tape_ptr = if tape_ptr - 1 < tape.len() { tape_ptr - 1 } else { 0 },
            b'.' => { stdout().write(&[tape[tape_ptr]]).expect("Could not write to stdout"); () },
            b',' => {
                let mut buffer: [u8; 1] = [0; 1];

                stdin().read(&mut buffer[..]).expect("Could not read from stdin");
                tape[tape_ptr] = if buffer[0] == b'\n' { 0 } else { buffer[0] };
            },
            b'[' => {
                if tape[tape_ptr] == 0 {
                    let mut count: usize = 1;
                    while count > 0{
                        inst_ptr += 1;
                        match program[inst_ptr] {
                            b'[' => count += 1,
                            b']' => count -= 1,
                            _ => (),
                        }
                    }
                }
            },
            b']' => {
                if tape[tape_ptr] != 0 {
                    let mut count: usize = 1;
                    while count > 0 {
                        inst_ptr -= 1;
                        match program[inst_ptr] {
                            b'[' => count -= 1,
                            b']' => count += 1,
                            _ => (),
                        }
                    }
                }
            },
            _ => (),
        }

        inst_ptr += 1;
    }

    tape_ptr
}

struct Options {
    verbose: bool,
    tape_dump: bool,
    tape_length: usize,
    filename: Option<String>,
    code: Option<String>,
}

fn main() {
    let mut options = Options {
        verbose: false,
        tape_dump: false,
        tape_length: 1000,
        filename: None,
        code: None,
    };


    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Simple brainfuck interpreter. If no filename or code given, reads it from stdin.");
        ap.refer(&mut options.tape_dump)
            .add_option(&["-d", "--dump"], StoreTrue,
            "Print memory at the end");
        ap.refer(&mut options.filename)
            .add_argument("filename", StoreOption,
            "Filename to read bf from");
        ap.refer(&mut options.code)
            .add_option(&["-e", "--eval"], StoreOption,
            "Eval given brainfuck code");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Say everything you do");
        ap.parse_args_or_exit();
    }

    let program: String = if let Option::Some(filename) = options.filename {
        if options.verbose { println!("Reading program from {}", filename) };
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file")
    } else if let Option::Some(code) = options.code {
        if options.verbose { println!("Reading program from command line") };
        code
    } else {
        if options.verbose { println!("Reading program from stdin") };
        let mut code: String = String::new();
        stdin().read_to_string(&mut code)
            .expect("Could not read code from stdin");
        code
    };

    if options.verbose { println!("Running bf program {}", program) };

    let mut tape: Vec<u8> = vec![0; options.tape_length];

    let tape_ptr: usize = execute(&program.as_bytes(), &mut tape);

    if options.tape_dump {
        println!("\nTape dump:");
        for x in 0..40 {
            print!("{:>4}", &tape[x]);
        }
        println!("\n{blank:>width$}^", blank = "", width = (tape_ptr + 1) * 4 - 1);
    }
}
