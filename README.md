# lox-rs
Rust implementation of the Lox programming language from
https://craftinginterpreters.com/.

## Run Lox

You can run Lox with the following command:
```sh
$ cargo run --bin lox
```

If all goes well, you should see the following output:
```
== test chunk ==
0000  123 OP_CONSTANT         0 '42'
0001    | OP_RETURN
```