// 001. Hello, crates, and main
//
// An executable is a binary crate with a function named main. That function is where the
// program starts. println! is a macro: the ! marks it. It writes a line to stdout. //
// starts a comment. Statements end with a semicolon.
//
// Cargo is the build tool. cargo run --bin 001_hello compiles this file and runs it.
//
//
// eprintln! writes to stderr. A binary can use both streams: stdout for the result,
// stderr for diagnostics.
//
// Run: cargo run --bin 001_hello

fn main() {
    println!("Hello, Rust");
    eprintln!("hello on stderr");
}
