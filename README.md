# lox-rs
Rust implementation of the Lox programming language from
https://craftinginterpreters.com/.

## Run Lox

You can either run the lox REPL or interpret a source file directly:
```sh
$ cargo run --bin lox               # REPL
$ cargo run --bin lox -- <FILE>     # Run file
```

At the moment, in both those modes, lox only converts the source into a Token
stream and prints it.