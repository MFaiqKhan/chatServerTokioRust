# Creating a Chat Server with async Rust and Tokio.
## With Detailed Explanation in the code.

Tokio library in Rust allows us to use asynchronous operations is __Rust__

It is not very feasible to write an entire program in different programming language, we can swap the little piece that is
actually being problematic and switch it in better language .

By default rust_lang has no idea how multiple async computations should be executed , so in order to run any async
computation we need to use a executor that knows how to run a future and drive it to completion .
futures in rust is basically a computation that may not have completed yet , it behaves like a promise in js but
with zero cost abstraction .

Rust know how to generate a future buut it does not know how to execute it , here we need to write __#[tokio::main]__ macro
which will create a tokio runtime and run the future on it .

## How to run the code

```bash
cargo run
```

## How to run the two servers

run this on two different terminals


```bash
telnet localhost 8080
```
and then type anything and press enter , you will see the same message on the other terminal
.
A basic chat server is ready to use .
