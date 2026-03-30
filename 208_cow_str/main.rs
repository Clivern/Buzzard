// 208. String vs &str recap
//
// &str is a view. String owns a buffer. to_string and to_owned build a String from &str.
// as_str borrows a String as &str. Deref makes most &str methods work on String. Prefer
// &str in function arguments so callers can pass literals.
//
// Run: cargo run --bin 208_cow_str

fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    let owned = String::from("rust");
    println!("{} {}", shout("hi"), shout(&owned));
}
