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
