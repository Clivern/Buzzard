// 024. Statements and expressions
//
// Rust is expression-oriented. if and match produce values. A block { } produces the
// value of its last expression. A semicolon turns an expression into a statement and
// throws the value away, leaving (). let is a statement, not an expression.
//
// Run: cargo run --bin 024_expressions

fn main() {
    let n = 3;
    let label = if n > 2 { "big" } else { "small" };
    let doubled = {
        let x = n;
        x * 2
    };
    println!("{label} {doubled}");
}
