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

## Variables and Mutability
- All variables immutable by default, defined by `let`
- Use `mut` keyword to make them mutable
- Constants are the same as in other languages and defined using `const`
- Variables can be shadowed by declaring them again using let keyword, refer [shadowing](./src/shadow/main.rs)
- Shadowing can allow us to redefine an immutable variable and then still have it as immutable if we dont use mut
- Shadowing allows changing a variables type, but a mutable variable cant change its type

## Data Types
- Scalar types
  - Integer
    - i8 or u8 -> signed or unsigned
    - i16 or u16 -> signed or unsigned
    - i32 or u32 -> signed or unsigned (i32 is default)
    - i64 or u64 -> signed or unsigned
    - i128 or u128 -> signed or unsigned
    - isize or usize -> signed or unsigned (size takes the max supported by system architecture)
    - On release builds rust wraps around integer values on overflow and causes program to "panic" at runtime in debug builds
    - Rust also provides some methods to gracefully handle wrapping as needed on runtime using
      - wrapping_*
      - checked_*
      - overflowing_*
      - saturating_*
  - Float
    - f32
    - f64 -> default
  - Boolean
    - bool
  - Character
    - char (use single quotes)
    - char in rust is 4 bytes and not 1 byte like other languages
    - It can represent unicode chars
- Compound types
  - Tuple
    - Bunch of different types of values together
    - Concept same as python tuples (cannot be modified / grown / shrunk)
    - eg `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    - To extract values use `let (x, y, z) = tup;` -> destructuring
    - Can also do `tup.0` or `tup.1` or `tup.2` to access values
    - A tuple without any values has a special name, unit written as `()` represents empty value or return type
  - Array
    - Have fixed length
    - eg `let a = [1, 2, 3, 4, 5];` or `let a: [i32; 5] = [1, 2, 3, 4, 5];`
    - `let a = [3; 5];` -> creates an array with five 3's
    - Individual elements accessed the same way as other languages

## Functions
- Pretty much same as other languages, just need to specify types of params
```rust
fn main() {
  // the {} block scope itself is an expression
  // expressions evaluate and return value
  // statements do not return a value
  // also rust does not allow
  // x = y = b kind of syntax since assignment not an 
  // expression and does not return the assigned value
  let y = {
      let x = 3;
      // note that if we add a semicolon here this becomes
      // a statement and wont return the value
      // resulting in a compile time error
      // any expression in rust does not end in semicolon
      x + 1
  };

  println!("The value of y is: {y}");
}
```
- Function return types in rust specified same way as python
- To return a value in rust its just an expression
- So we can just write
```rust
// do this
fn add(x: i32, y: i32) -> i64 {
  x + y
}

// instead of
// return keyword does exist and work
// but is generally omitted if at the end
fn add(x: i32, y: i32) -> i64 {
  return x + y;
}
```

## Control
- check code and comments [control](./src/control/main.rs)

## Practice
- [Temperature converter](./src/temperature_converter/src/main.rs)
- [Nth fibonacci](./src/nth_fibonacci/src/main.rs)

## Ownership
- All heap data must be owned by exactly one variable (Use Box to own some memory on heap)
- Rust deallocates heap data once its owner goes out of scope
- Ownership can be transferred by moves, which happen on assignments and function calls
- Heap data can only be accessed through its current owner, not a previous owner (if you reassign a heap variable then dont use older one since it gets deallocated)

## Referencing and Borrowing
- [Referencing](./src/referencing/main.rs)
- Dereferencing is same as cpp using `*`
- For boxes or boxlike variables rust does not allow aliasing or assignment (if you assign then ownership gets transferred and old variable deallocated)
- Better to refer [this](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html) no point in writing when examples are self explanatory