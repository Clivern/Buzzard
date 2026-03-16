// 148. String methods
//
// push, push_str, replace, split, trim, contains, starts_with, to_lowercase cover most
// text work. String is UTF-8. Indexing with a byte range is allowed; indexing a single
// byte as a char is not. split returns an iterator.
//
// Run: cargo run --bin 148_strings

fn main() {
    let mut s = String::from(" Hello ");
    s.push_str("Rust");
    println!("{}", s.trim().to_lowercase());
    println!("{:?}", "a,b,c".split(',').collect::<Vec<_>>());
    println!("{}", "hello".replace("l", "r"));
}
