<p align="center">
    <img alt="Logo" src="https://i.imgflip.com/3l3hse.jpg" width="60%" />
    <h3 align="center">Buzzard</h3>
    <p align="center">A hands-on tour of Rust, one small runnable lesson at a time.</p>
</p>


## What's inside

Buzzard is a learning playground: 60 numbered Rust lessons, one topic each. Every lesson
lives in its own folder with a `main.rs` that opens with an explanation of the topic, then
works through it in a series of small `demo_*` functions, each printing a `-- section --`
header so the output is easy to follow. Code the compiler rejects is shown commented out,
with the real error it produces. Each lesson runs on its own:

```zsh
$ cargo run --bin 01_print_format
$ cargo check --bin 01_print_format   # type-check only, faster
$ cargo test --bin 54_tests_docs      # lessons with tests
```

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
- [Standard library docs](https://doc.rust-lang.org/std/index.html)
