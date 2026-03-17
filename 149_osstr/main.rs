// 149. OsString and OsStr
//
// Paths and args are OsString on Windows they may not be UTF-8. to_str() is Option.
// to_string_lossy() replaces bad bytes. From<&str> works when you have UTF-8. Keep
// OsString until you actually need a String.
//
// Run: cargo run --bin 149_osstr

use std::ffi::OsStr;

fn main() {
    let p = OsStr::new("file.txt");
    println!("{} {:?}", p.to_string_lossy(), p.to_str());
}
