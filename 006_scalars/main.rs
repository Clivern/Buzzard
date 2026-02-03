// 006. Scalar types
//
// Rust's scalar types are integers, floats, booleans, and char. Integers have a
// signedness and a width: i32, u8, usize. Floats are f32 and f64. bool is true or false.
// char is a Unicode scalar value, written in single quotes, and is four bytes. A type
// suffix like 10u8 pins the type when inference is not enough.
//
// Run: cargo run --bin 006_scalars

fn main() {
    let a: i32 = -3;
    let b: u8 = 10;
    let c = 2.5_f64;
    let ready: bool = true;
    let letter: char = 'R';
    let snow: char = '☃';
    println!("{a} {b} {c} {ready} {letter} {snow}");
}
