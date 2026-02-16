// 048. String slices
//
// &str is a borrowed string slice. String is an owned growable buffer. A string literal
// has type &'static str. s[0..4] must land on UTF-8 boundaries. chars() and get() are
// safer than raw indexing when the text is not ASCII.
//
// Run: cargo run --bin 048_str_slices

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let owned = String::from("hello rust");
    println!("{}", first_word(&owned));
    println!("{}", first_word("only"));
}
