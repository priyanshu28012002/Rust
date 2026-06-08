Installing rustup on Linux or macOS

```BASH
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustc --version
rustup update
rustup self uninstall

```

## Build and run

```bash

rustc main.rs
./main

```

## Creating a Project with Cargo

```bash
cargo new hello_cargo
cd hello_cargo
```


## Filename: Cargo.toml 
(Tom’s Obvious, Minimal Language)
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"
[dependencies]
```
## Building and Running a Cargo Project
Building for Release
When your project is finally ready for release, you can use cargo build --release to compile
it with optimizations. This command will create an executable in target/release instead of
target/debug. The optimizations make your Rust code run faster, but turning them on lengthens
the time it takes for your program to compile.

```bash
cargo build
Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs


cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running `target/debug/hello_cargo`
Hello, world!

cargo build --release


```