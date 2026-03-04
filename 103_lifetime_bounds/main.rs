// 103. Lifetime bounds
//
// T: 'a means all references in T are valid for 'a. It appears on structs that hold
// generic data which might contain borrows, and on trait objects: Box<dyn Trait + 'a>.
// 'a: 'b means 'a lives at least as long as 'b.
//
// Run: cargo run --bin 103_lifetime_bounds

fn print_ref<'a, T: std::fmt::Display + 'a>(value: &'a T) {
    println!("{value}");
}

fn main() {
    let s = String::from("hi");
    print_ref(&s);
}
