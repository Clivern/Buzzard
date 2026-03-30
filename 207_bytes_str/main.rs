// 207. from_utf8
//
// str is always valid UTF-8. from_utf8 checks a byte slice. from_utf8_lossy replaces bad
// sequences with U+FFFD. from_utf8_unchecked is unsafe. String is a Vec<u8> that
// maintains the UTF-8 invariant. into_bytes gives the raw vec.
//
//
// String::from_utf8 takes a Vec<u8> and returns Result, giving the bytes back on failure
// so you do not lose them.
//
// Run: cargo run --bin 207_bytes_str

fn main() {
    let ok = std::str::from_utf8(b"hi");
    let bad = std::str::from_utf8(&[0xff]);
    println!("{ok:?} {bad:?}");
    println!("{}", String::from_utf8_lossy(&[0xff]));
    println!("{:?}", String::from_utf8(vec![b'o', b'k']));
}
