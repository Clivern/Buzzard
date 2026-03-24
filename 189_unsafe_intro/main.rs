// 189. unsafe
//
// unsafe is a keyword that lets you call unsafe functions, dereference raw pointers,
// access mut statics, and implement unsafe traits. It does not turn off the borrow
// checker for safe references. Keep unsafe blocks tiny and write a Safety comment that
// lists the invariants you upheld.
//
//
// Keep the unsafe block to the actual dereference. Building the pointer from a reference
// is safe and does not need the keyword.
//
// Run: cargo run --bin 189_unsafe_intro

fn main() {
    let n = 5;
    let p: *const i32 = &n;
    let v = unsafe {
        // Safety: p came from a live &i32, so it is aligned, non-null, and in bounds.
        *p
    };
    println!("{v}");
    println!("ptr {:p}", p);
}
