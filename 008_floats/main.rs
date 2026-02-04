// 008. Floating point
//
// f64 is the default float. Division of floats keeps the fraction. Equality on floats is
// often the wrong test because of rounding; total_cmp orders them, and is_nan reports
// NaN. 0.1 + 0.2 is not exactly 0.3 in binary floating point.
//
// Run: cargo run --bin 008_floats

fn main() {
    let x = 7.0 / 2.0;
    let y: f32 = 1.0 / 3.0;
    println!("f64 {x} f32 {y}");
    println!("nan {} inf {}", f64::NAN.is_nan(), f64::INFINITY.is_infinite());
    println!("0.1 + 0.2 == 0.3? {}", (0.1 + 0.2) == 0.3);
}
