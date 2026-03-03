// 100. Lifetimes in structs
//
// A struct that holds a reference needs a lifetime on the struct: struct Excerpt<'a> {
// s: &'a str }. The struct cannot outlive the data it borrows. This is how iterators
// hold a borrow of the collection they walk.
//
// Run: cargo run --bin 100_lifetime_structs

struct Excerpt<'a> {
    s: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael.");
    let first = novel.split('.').next().unwrap();
    let e = Excerpt { s: first };
    println!("{}", e.s);
}
