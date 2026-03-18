// 158. Doc comments
//
// /// documents the next item. //! documents the enclosing module. Code in ```rust
// fences in docs is a doctest: cargo test runs it. # hide a line from the rendered docs
// but keep it for the test. Intra-doc links are [Type].
//
// Run: cargo run --bin 158_doc_comments

//! A tiny documented binary.

/// Double `n`.
///
/// ```
/// assert_eq!(2 * 2, 4);
/// ```
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", double(3));
}
