// 183. Wrapping errors
//
// map_err wraps an error with context. From lets ? convert. Box<dyn Error + Send + Sync>
// is a common catch-all in binaries. keep the source() chain so Display of the outer
// error can still mention the inner one.
//
// Run: cargo run --bin 183_error_chain

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct LoadError(String);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "load: {}", self.0)
    }
}

impl Error for LoadError {}

fn load() -> Result<i32, LoadError> {
    "x".parse::<i32>().map_err(|e| LoadError(e.to_string()))
}

fn main() {
    println!("{:?}", load());
}
