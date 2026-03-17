// 154. use
//
// use brings a path into scope. use a::b::c as d renames. use a::{b, c} is a group. use
// a::* is a glob; keep it rare. A use in a function is local to that function. Re-export
// with pub use so callers see a shorter path.
//
// Run: cargo run --bin 154_use_paths

use std::collections::HashMap as Map;
use std::io::{self, Write};

fn main() {
    let mut m: Map<&str, i32> = Map::new();
    m.insert("n", 1);
    writeln!(io::stdout(), "{m:?}").unwrap();
}
