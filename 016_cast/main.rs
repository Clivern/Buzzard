// 016. as casts
//
// as converts between primitive types. Narrowing a float truncates toward zero. Casting
// u16 to u8 keeps the low 8 bits. Casting a pointer or a float to an integer is allowed
// for primitives; most useful conversions go through From and TryFrom instead, which are
// checked or lossless.
//
// Run: cargo run --bin 016_cast

fn main() {
    let n: f64 = 3.9;
    let truncated = n as i32;
    let wide: u16 = 300;
    let narrow = wide as u8;
    println!("trunc {truncated} narrow {narrow}");
}
