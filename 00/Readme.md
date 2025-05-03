# Rust Installation

https://www.rust-lang.org/tools/install

# RustUp

- rustup is a utility tool that runs in your command line.
- rustup enable us to update the version of rust also to uninstall rust
- rustup also help us to see rust official documentation

Command :
'''rust

> rustup

'''

To update rust - rustup update
To uninstall rust - rustup self uninstall
To view rust documentation on local computer - rustup doc

# Install Git

# Create Rust Project with Cargo

- To create a rust project using cargo

> cargo new project_name

- the project name should be written in snakecase i.e. hello_world

'''

> cargo new hello_world
> Creating binary (application) `hello_world` package
> note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

'''

- the next line show ' Creating binary crate '
- there 2 types of crate,

1. binary crate - it is a standalone crate

2) library crate - it is incorporated in other binary crates

# To run the file

- using rustc

> cd src
> rustc main.rs
> ./main

# Formatting using rustfmt and cargo fmt

- using rustfmt

  > rustfmt main.rs

- using

  > cd ..
  > cargo fmt

# cargo build

- to build a program

> cargo build

or

> cargo b

- to build a optimised version of program

> cargo build --release

or

> cargo build --r

- to reset the cache data for building from scratch

> cargo clean

- to build and run the program immediately

> cargo run

or

> cargo r

- to check the code for any voilation

> cargo check
