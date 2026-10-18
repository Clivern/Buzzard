<p align="center">
    <img alt="Logo" src="https://i.imgflip.com/3l3hse.jpg" width="60%" />
    <h3 align="center">Buzzard</h3>
    <p align="center">A hands-on tour of Rust basics, written as runnable, self-checking code.</p>
    <p align="center">
        <a href="https://github.com/Clivern/Buzzard/actions/workflows/build.yml">
            <img src="https://github.com/Clivern/Buzzard/actions/workflows/build.yml/badge.svg" alt="Build" />
        </a>
    </p>
</p>


## What's inside

Buzzard is a learning playground. Most examples live in [`src/main.rs`](src/main.rs) and
use `assert_eq!` to check their own results, so `cargo run` doubles as a test: if a
concept is explained wrong, the program panics.

Topics covered, in order:

- Variables, mutability, shadowing and constants
- Primitive and compound types (tuples, arrays, slices)
- Control flow: `if`, `loop`, `while`, `for`, `match`, `if let`
- Ownership, moves, cloning, borrowing and mutable references
- Strings and string slices
- Structs, tuple structs and methods
- Enums, `Option` and `Result`
- Vectors and hash maps
- Generics, traits, default implementations and `impl Trait`
- Lifetimes
- Iterators and closures
- Modules and visibility (`pub`, `pub(crate)`, `pub(super)`, `pub(in path)`), including
  modules split across files ([`src/front_house.rs`](src/front_house.rs),
  [`src/sub/`](src/sub))
- Using an external crate (`rand`)


## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
- [Standard library docs](https://doc.rust-lang.org/std/index.html)
