// 045. Mutable references
//
// &mut T is a unique reference. While it is alive, nothing else can read or write that
// value. Use it when a function should update the caller's data. You write *r = ... to
// replace the value, or call methods that take &mut self.
//
// Run: cargo run --bin 045_mut_refs

fn push_bang(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("hi");
    push_bang(&mut s);
    println!("{s}");
}
