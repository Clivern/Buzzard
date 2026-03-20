// 170. ref patterns
//
// ref in a pattern binds a reference instead of moving. ref mut binds &mut. Matching on
// a struct without ref would move fields. You can also write & in the pattern: Some(x)
// on an &Option is auto-deref; Some(&x) on &Option<T> is different from Some(x) on
// Option<&T>.
//
// Run: cargo run --bin 170_ref_patterns

fn main() {
    let s = Some(String::from("hi"));
    match &s {
        Some(v) => println!("{v}"),
        None => {}
    }
    println!("{:?}", s);
}
