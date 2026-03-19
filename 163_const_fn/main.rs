// 163. const fn
//
// const fn can be called in const contexts: array lengths, const items, type layout. Not
// every operation is allowed inside: no heap, limited loops. Regular runtime calls still
// work. const blocks (edition 2024) evaluate a block at compile time.
//
// Run: cargo run --bin 163_const_fn

const fn square(n: i32) -> i32 {
    n * n
}

const N: i32 = square(5);

fn main() {
    println!("{N} {}", square(3));
}
