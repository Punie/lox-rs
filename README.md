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
0000  123 OP_CONSTANT         0 '1.2'
          [ 1.2 ]
0001    | OP_CONSTANT         1 '3.4'
          [ 1.2 ][ 3.4 ]
0002    | OP_ADD
          [ 4.6 ]
0003    | OP_CONSTANT         2 '5.6'
          [ 4.6 ][ 5.6 ]
0004    | OP_DIVIDE
          [ 0.8214285714285714 ]
0005    | OP_NEGATE
          [ -0.8214285714285714 ]
0006    | OP_RETURN
-0.8214285714285714
```