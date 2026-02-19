// 062. The ? operator
//
// ? on a Result returns the error from the current function, after converting it with
// From. ? on Option returns None early. The function's return type must be compatible.
// main can return Result<(), Box<dyn Error>> so ? works at the top level.
//
// Run: cargo run --bin 062_question

use std::num::ParseIntError;

fn add_texts(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

fn main() {
    println!("{:?} {:?}", add_texts("2", "3"), add_texts("2", "x"));
}
