<p align="center">
    <img alt="Logo" src="https://i.imgflip.com/3l3hse.jpg" width="60%" />
    <h3 align="center">Buzzard</h3>
    <p align="center">A hands-on tour of Rust, one small runnable lesson at a time.</p>
</p>


## What's inside

Buzzard is a learning playground: 210 small, numbered Rust lessons. Each one lives in its
own folder with a commented `main.rs`, and runs on its own:

```zsh
$ cargo run --bin 001_hello
```

Topics, in order:

- Basics: variables, mutability, constants, shadowing, scalar and compound types
- Functions, expressions and control flow (`if`, `loop`, `while`, `for`, `match`, `let else`)
- Ownership, moves, `Clone`/`Copy`, borrowing, slices
- Structs, enums, `Option`, `Result` and error handling
- Conversion and core traits (`From`, `TryFrom`, `Display`, `Eq`/`Ord`, `Hash`, `Drop`, `Deref`, `Cow`)
- Generics, traits, `impl Trait` and `dyn Trait`
- Iterators, closures and lifetimes
- Collections: `Vec`, `HashMap`, `BTreeMap`, `VecDeque`, `BinaryHeap` and more
- Smart pointers and interior mutability: `Box`, `Rc`, `Weak`, `Cell`, `RefCell`, `Arc`
- Concurrency: threads, `Mutex`, `RwLock`, channels, atomics, `Condvar`, `Barrier`
- Files, paths, env, args, processes, stdin, time
- Modules, visibility, `cfg`, tests, doc comments and macros
- Advanced patterns, const generics, `PhantomData`, operator overloading
- Async, `Pin`, unsafe, raw pointers, FFI and `repr`


## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
- [Standard library docs](https://doc.rust-lang.org/std/index.html)
