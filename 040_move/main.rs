// 040. Moves into functions
//
// Passing an owned value into a function moves it, unless the type is Copy. After
// take(s), the caller cannot use s. Return the value if the caller still needs it, or
// take a reference if the function only needs to look.
//
// Run: cargo run --bin 040_move

fn take(s: String) {
    println!("took {s}");
}

fn give_back(s: String) -> String {
    s
}

fn main() {
    let s = String::from("hi");
    take(s);
    let t = String::from("there");
    let t = give_back(t);
    println!("{t}");
}
