# Rustbook notes

## 1 Getting started

### 1.1 Installation

#### Commands

Rust installation in Mac:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

C installation in Mac:

```bash
xcode-select --install
```

Rust update:

```bash
rustup update
```

Rust uninstall:

```bash
rustup self uninstall
```

### 1.2 Hello, World

Rust main function.

```rust
fn main() {
    println!("Hello, world!");
}
```

File compilation.

```bash
rustc main.rs
```

Execute program.

```bash
./main
```

### 1.3 Hello, Cargo

Cargo is the Rust tooling application.

To check **cargo** version.

```bash
cargo --version
```

Create a new Rust project using cargo.

```bash
cargo new hello_cargo
cd hello_cargo
```

Create a new Rust project using cargo with git as vcs.

```bash
cargo new --vcs=git hello_cargo
cd hello_cargo
```

Build a Rust project using cargo.

```bash
cargo build
```

Execute a Rust project using cargo.

```bash
cargo run
```

Check a project compiles without compiling

```bash
cargo check
```

Build a Rust project using cargo for releasing.

```bash
cargo build --release
```

## 2 Guessing game

The **use** keyword imports Rust crates or libraries.

```rust
use std::io;
```

Create a **variable**

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

Dependencies can be added in **Cargo.toml** file.

```toml
[dependencies]
rand = "0.8.5"
```

Dependencies can be added using cargo add and updated with cargo update.

```bash
cargo add <crate>
cargo update
```

Creates local crate documentation.

```bash
cargo doc --open
```

### Use of loop, break, continue, match, enum, and Result

- **loop** keyword is used to create infinite loops.
- **break** keyword break the current loop execution.
- **continue** keyword skips to the next loop execution.
- **match** keyword compares values in a variable, prticularly in Enums, Results, and Structs.
- **enum** is a enumeration in Rust.
- **Result** is a generic data type which represents a value or an Error.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

---

## 7 Managing Growing Projects with Packages, Crates, and Modules

The **module system**, include:

- **Packages**: A Cargo feature that lets you build, test, and share crates.
- **Crates**: A tree of modules that produces a library or executable.
- **Modules** and use: Let you control the organization, scope, and privacy of paths.
- **Paths**: A way of naming an item, such as a struct, function, or module.

### Packages and Crates

A **crate** is the smallest amount of code that the Rust compiler considers at a time.

A crate can be: a **binary crate** or a **library crate**.

**Binary crates** are programs, you can run and have a **function main**.

**Library crates** don’t have a main function, they **define functionality intended to be shared** with multiple projects.

A **package is a bundle of crates** that provides a set of functionality. A **package contains a Cargo.toml** file that describes how to build those crates. 