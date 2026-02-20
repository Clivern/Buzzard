// 068. FromStr
//
// FromStr parses a &str into a value. parse() is a convenience method that calls it. The
// associated Err type reports why parsing failed. Many std types implement it: integers,
// bool, IpAddr, SocketAddr.
//
// Run: cargo run --bin 068_fromstr

use std::str::FromStr;

fn main() {
    let n = i32::from_str("42").unwrap();
    let flag = bool::from_str("true").unwrap();
    println!("{n} {flag}");
}
