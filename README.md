# rlisp
A simple Lisp interpreter written in Rust.

## What dialect of Lisp does it recognize?
This interpreter does not adhere to any specific Lisp-like specification. Nevertheless, its syntax closely resembles a 
simplified version of the Scheme variant.

## Why?
Why not? In all seriousness, this project is not destined to be a commercial intepreter. Instead it aims to be a
simple and modifiable piece of software to assist the teaching of compiler/interpreter engineering.

## How can I try it?
Just clone the sources and type `cargo run` inside the repository folder. By default, the interpreter reads and executes
`test-code.scm` but changing that should be trivial (simply edit `src/main.rs`).
