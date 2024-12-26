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
