// 018. Comments and docs
//
// // comments out the rest of the line. /* */ can span lines. /// is a doc comment on
// the next item; //! documents the enclosing parent. cargo doc renders them. Keep
// comments for why, and let the code show what.
//
// Run: cargo run --bin 018_comments

/// Adds one. Doc comments become HTML on docs.rs and cargo doc.
fn bump(n: i32) -> i32 {
    n + 1
}

fn main() {
    // line comment
    let n = /* block */ bump(3);
    println!("{n}");
}
