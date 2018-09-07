# BF Rust

A small brainf*ck interpreter to get me into rustlang, I plan to improve this interpreter and my knowledge of Rust as time goes on.

## Usage

Be aware that the given code must be clean, meaning every [ has a matching ] and no argument checking is being done yet.

```
sh $ ./bf-interpreter --help
Usage:
    ./bf-interpreter [OPTIONS] [FILENAME]

Simple brainfuck interpreter. If no filename or code given, reads it from
stdin.

positional arguments:
  filename              Filename to read bf from

optional arguments:
  -h,--help             show this help message and exit
  --color               Color stuff on ANSI terminals
  -D,--dump             Print memory at the end
  -e,--eval EVAL        Eval given brainfuck code
  -v,--verbose          Say everything you do
  -x                    Enables Extended Type I to III (1-3) features
  -s,--size SIZE        Changes the memory tape size, defaults to 1000
```