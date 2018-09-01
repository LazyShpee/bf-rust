extern crate argparse;
extern crate colored;

use argparse::{ArgumentParser, StoreTrue, StoreOption};

mod interpreter;

use interpreter::{Options, VM};

/**
 * This function assumes &program is CLEAN,
 * meaning every [ has a matching ] and no [] is present
 */

fn main() {
    let mut options = Options {
        verbose: false,
        tape_dump: false,
        color: false,
        tape_length: 18,
        filename: None,
        code: None,
    };


    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Simple brainfuck interpreter. If no filename or code given, reads it from stdin.");

        ap.refer(&mut options.color)
            .add_option(&["--color"], StoreTrue,
            "Color stuff on ANSI terminals");
        ap.refer(&mut options.tape_dump)
            .add_option(&["-D", "--dump"], StoreTrue,
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

    let mut vm = VM::new(&mut options);

    vm.run();

    if options.tape_dump {
        vm.dump(interpreter::DumpMode::Hex, options.color);
    }
}
