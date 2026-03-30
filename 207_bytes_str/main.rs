// 207. from_utf8
//
// str is always valid UTF-8. from_utf8 checks a byte slice. from_utf8_lossy replaces bad
// sequences with U+FFFD. from_utf8_unchecked is unsafe. String is a Vec<u8> that
// maintains the UTF-8 invariant. into_bytes gives the raw vec.
//
// Run: cargo run --bin 207_bytes_str

fn main() {
    let ok = std::str::from_utf8(b"hi");
    let bad = std::str::from_utf8(&[0xff]);
    println!("{ok:?} {bad:?}");
    println!("{}", String::from_utf8_lossy(&[0xff]));
}
