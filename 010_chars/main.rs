// 010. Characters and Unicode
//
// char is a Unicode scalar value, not a byte and not a grapheme cluster. 'é' is one
// char. Some user-visible letters are several chars (e.g. e + combining acute). Escape
// sequences include \n, \t, \u{1F980}. chars() on a string yields char values. len() on
// a string counts bytes, not chars.
//
// Run: cargo run --bin 010_chars

fn main() {
    let crab: char = '🦀';
    println!("crab {crab} as u32 {}", crab as u32);
    let s = "Go → 世";
    println!("bytes {} chars {}", s.len(), s.chars().count());
    for c in s.chars() {
        println!("{c} U+{:04X}", c as u32);
    }
}
