// 065. panic
//
// panic! unwinds the stack (by default) and aborts the thread. Use it for bugs: out-of-
// range index, broken invariants. Do not use it for expected I/O failure. catch_unwind
// can catch a panic in the same thread, but it is not a substitute for Result.
//
// Run: cargo run --bin 065_panic

fn main() {
    let result = std::panic::catch_unwind(|| {
        panic!("boom");
    });
    println!("caught {}", result.is_err());
}
