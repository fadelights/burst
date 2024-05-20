# Prerequisites
To make sure that Rust is installed on your system, you can run
```shell
$ rustc --version
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```
in your command line. Visit
[the installation guide](https://www.rust-lang.org/tools/install)
for more info.

A number of other useful tools are installed
with Rust, some of which include:
- `rustc`: Compiler
- `rustup`: Version manager
- `cargo`: Package manager
- `rustfmt`: Code formatter
- `rust-gdb`: Debugger
- `rust-analyzer`: Language server (used by IDEs and editors)

You may update your version of Rust by running
```shell
$ rustup update
```

To view the documentation offline, you can run
```shell
$ rustup doc
```

# The First Steps
You can put your Rust source codes in a plain text file
with the (optional) `.rs` extension; e.g. `hello.rs`.

The main body of a rust program is the `main` function.
```rust
fn main() {
    println!("Hello, Rust!");
}
```

# Compiling Your Code
Compiling your source file is as easy as running
```shell
$ rustc hello.rs
```
However, for larger projects with many files, the standard way of
compiling your project is using Cargo.

# Hello, Cargo!
Cargo is Rust’s build system and package manager. You can use
this tool to manage your Rust project. Some editors (like VS Code)
also require your code to be part of a Cargo project to be able to
provide features such as code completion, debuggin, etc.

- Creating a project with Cargo named `hello`
    ```shell
    $ cargo new hello
    ```
- Creating a project with Cargo named `hello` in the `00-hello`
directory
    ```shell
    $ cargo new --name hello 00-hello
    ```

Here is the basic layout of a Cargo project directory:
```
hello
├── src/
│   └── main.rs
├── target/
│   ├── debug/
│   └── release/
└── Cargo.toml
```

Your source code goes into the `src/` directory.
Once your compile it, the executable will be available in `target/`.
The `Cargo.toml` file contains info about the project's configuration
and dependencies. It can be used to reproduce your project in another
environment such as your buddy's new fancy MacBook Pro(TM).

Here's a quick rundown of available Cargo commands:
- `cargo new`: Creates a new project
- `cargo build`: Builds (compiles) your project --
this is the quick way to compile your source code for rapid
development and testing. Places the executable in `target/debug/`
- `cargo build --release`: Builds your project for release --
takes longer to compile but does more optimizations
Places the executable in `target/release/`
- `cargo run`: Builds and runs your project
- `cargo check`: Checks if your code will compile or not

By using `cargo run`, you don't have to manually build and run your
code in separate steps anymore.
