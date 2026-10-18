<p align="center">
    <img alt="Logo" src="https://i.imgflip.com/3l3hse.jpg" width="60%" />
    <h3 align="center">Buzzard</h3>
    <p align="center">A hands-on tour of Rust, one small runnable lesson at a time.</p>
</p>


## What's inside

60 small Rust lessons, one topic each, from the basics to ownership, traits, concurrency,
async, and FFI.

Each folder holds a commented `main.rs` you can run on its own:

```zsh
$ cargo run --bin 01_print_format
```

## Website

Read the lessons with their output at [buzzard.clivern.com](https://buzzard.clivern.com).
To build and preview it locally:

```zsh
$ python3 site/build.py
$ python3 -m http.server -d docs
```

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
- [Standard library docs](https://doc.rust-lang.org/std/index.html)
