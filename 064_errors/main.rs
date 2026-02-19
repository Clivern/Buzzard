// 064. Custom errors
//
// A type is an error if it implements std::error::Error, which requires Display and
// Debug. From lets ? convert your inner errors. In small programs, Box<dyn Error> is
// enough. In libraries, an enum of cases is easier to match on.
//
// Run: cargo run --bin 064_errors

use std::fmt;

#[derive(Debug)]
struct EmptyName;

impl fmt::Display for EmptyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "empty name")
    }
}

impl std::error::Error for EmptyName {}

fn greet(name: &str) -> Result<String, EmptyName> {
    if name.is_empty() {
        return Err(EmptyName);
    }
    Ok(format!("hello {name}"))
}

fn main() {
    println!("{:?} {:?}", greet("Ada"), greet(""));
}
