# Rust

Learning from [Rust Book (Modified by Brown University)](https://rust-book.cs.brown.edu/)

## Intro
- high level with low level control over systems (like memory, threads)
- does most of the heavy lifting in compile step to prevent underlying bugs like concurrency

## Toolchain
- cargo -> package manager
- rustfmt -> code formatter aka prettier
  - `rustfmt filename` for a file or `cargo fmt` for entire code format
- rust language server -> code completion + lint

## CLI
- rustc -> compiler
  - `rustc --version` -> to check version
- rustup -> utility / toolchain 
  - `rustup update` -> update to latest
  - `rustup self uninstall` -> remove rust
  - `rustup doc` -> opens local docs in browser

## Hello World
```rust
// hello_world.rs
// file names generally contain underscore as separator
// fn to denote function definition
fn main() {
    // the function does have a !
    // strings must be in double quotes and char single quote (like java?)
    // indent with 4 spaces
    println!("Hello, World!"); 
    // ok so println! is not a function (its something called a macro)
    // and docs says macro dont follow same rules as functions dunno what that means for now
}
```

## Cargo
- package manager
- does things like
  - creating projects
  - code building
  - downloading deps
  - building deps
- `cargo --version` -> check version
- `cargo new APP_NAME` -> create new rust app with Cargo.toml (TOML - Tom’s Obvious, Minimal Language), main.rs and initialise as git
- `cargo build` -> build app
  - generates lock file for dependencies
  - generates a target folder for build
  - build executable present in `./target/debug/APP_NAME`
- `cargo run` -> build and run
- `cargo check` -> tries to compile and check for errors but does not generate an executable
- `cargo build --release` to build for prod with optimisations
  - output generated in `target/release` folder
- check [crates.io](https://crates.io/) for available crates to download and use
- `cargo doc --open` will open docs containing docs of all dependencies used in your project

## Guessing Game

- check code and comments [guessing_game](./src/guessing_game/src/main.rs)