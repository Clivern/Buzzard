// 011. Tuples
//
// A tuple groups values of mixed types. Access a field with a dot and an index: pair.0.
// () is the unit tuple, the type of a function that returns nothing useful. A tuple of
// one element needs a trailing comma: (1,). Destructuring binds the parts to names.
//
// Run: cargo run --bin 011_tuples

fn main() {
    let pair = (7, "ok");
    println!("{} {}", pair.0, pair.1);
    let (n, label) = pair;
    println!("unpacked {n} {label}");
    let unit = ();
    println!("unit {unit:?}");
}
