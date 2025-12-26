# rust-learning

This repository will closely follow the book [The Rust Programming Language](https://doc.rust-lang.org/book/) to help me learn Rust. It will contain various code examples and exercises from the book as well as my own small projects and ideas implemented in Rust.

We can also access the book offline by running (has to be run in PowerShell for Windows):

```sh
rustup docs --book
```

I've also turned off lookahead / intellisense for VSCode so I don't get things spoiled while trying to learn syntax.

## NOTES

1. Use `cargo new {project_name}` to create a new directory initialized for rust with cargo.
2. Run `cargo build` inside the directory to build the files.
3. Run `./target/debug/{project_name}` or `cargo run` to execute the binary file(s).
4. `cargo check` runs a quick check to make sure the code compiles without actually producing an executable.
5. `cargo build --release` will create a release executable in the target/release directory.
