# rscheme
A simple Scheme interpreter written in Rust.

## Why?
Why not? In all seriousness, this project is not destined to be a commercial intepreter. Instead it aims to be a
simple and modifiable piece of software to assist the teaching of compiler/interpreter engineering.

## How can I try it?
Just clone the sources and type `cargo run` inside the repository folder. By default, the interpreter reads and executes
`test-code.scm` but changing that should be trivial (simply edit `src/main.rs`).

## Features
- [x] Basic arithmetic operations.
- [x] Variables.
- [x] Lambda expressions.
- [x] Recursion.
- [ ] Scheme function declaration syntax.
- [ ] REPL prompt.
- [ ] `display` procedure.
- [ ] Lists.
- [ ] String manipulation procedures.
- [ ] Foreign function calls.
