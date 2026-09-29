// 05. Operators, casts, and parsing
//
// 7 / 2 is integer division, so the result is 3. 7.0 / 2.0 keeps the fraction. % is the
// remainder and takes the sign of the left side. << and >> shift bits; &, |, and ^ are
// bitwise and, or, and xor; ! is bitwise not on integers. Compound assignment such as +=
// updates a mut binding. There is no ++ or --.
//
// Rust never converts numbers implicitly. as converts between primitive types: it
// truncates floats toward zero and keeps the low bits when narrowing integers. From and
// TryFrom are the checked, lossless alternatives.
//
// str::parse goes the other way, from text to a value, through the FromStr trait. It
// returns a Result because the text might not be valid.
//
// Run: cargo run --bin 05_operators_casts

// Example 1: arithmetic.
fn demo_arithmetic() {
    println!("-- arithmetic --");

    println!("int div {} remainder {} float {}", 7 / 2, 7 % 2, 7.0 / 2.0);
    println!("negative div {} rem {}", -7 / 2, -7 % 2);
    println!("precedence {} {}", 2 + 3 * 4, (2 + 3) * 4);

    let mut n = 1;
    n += 4;
    n *= 3;
    n -= 1;
    n /= 2;
    n %= 4;
    println!("compound n {n}");

    // let x = 1 + 2.0;
    // error[E0277]: cannot add a float to an integer
    let x = 1 as f64 + 2.0;
    println!("explicit convert {x}");
}

// Example 2: comparison and logic.
fn demo_comparison() {
    println!("-- comparison --");

    let a = 3;
    let b = 5;
    println!("== {} != {} < {} >= {}", a == b, a != b, a < b, a >= b);
    println!("strings compare lexically: {}", "apple" < "banana");
    println!("logic {} {} {}", a < b && b < 10, a > b || b == 5, !(a == b));
}

// Example 3: bitwise operators and shifts.
fn demo_bits() {
    println!("-- bits --");

    println!("shift {} {} bits {} {} {}", 1 << 3, 8 >> 1, 5 & 3, 5 | 2, 5 ^ 1);
    println!("not {:#010b}", !0b1010_0000u8);

    // A small flag set packed into a u8.
    const READ: u8 = 0b001;
    const WRITE: u8 = 0b010;
    const EXEC: u8 = 0b100;

    let mut perms = READ | WRITE;
    println!("can write? {}", perms & WRITE != 0);
    perms &= !WRITE;
    perms |= EXEC;
    println!("perms {perms:03b} can write? {}", perms & WRITE != 0);
    println!("rotate {:#06x}", 0x1234u16.rotate_left(4));
}

// Example 4: as casts and what they do to the value.
fn demo_as_casts() {
    println!("-- as casts --");

    let n: f64 = 3.9;
    println!("trunc {} neg {}", n as i32, -3.9_f64 as i32);
    println!("saturating float cast {} {}", 300.0_f64 as u8, f64::NAN as i32);

    let wide: u16 = 300;
    println!("narrow {} (300 - 256)", wide as u8);
    println!("sign change {} {}", -1i32 as u32, u32::MAX as i32);
    println!("widen {} {}", 200u8 as i64, -5i8 as i32);
    println!("char {} {}", 'A' as u8, 97u8 as char);
    println!("bool {}", true as u8 + 1);
}

// Example 5: checked conversions with From and TryFrom.
fn demo_checked_conversions() {
    println!("-- From and TryFrom --");

    // From exists only where the conversion can never lose information.
    let a = i64::from(42i32);
    let b: f64 = 3u8.into();
    println!("from {a} into {b}");

    // TryFrom reports values that do not fit.
    println!("{:?} {:?}", u8::try_from(200i32), u8::try_from(300i32));

    let small: Result<i8, _> = 1000i64.try_into();
    println!("{small:?}");

    let index: usize = 7u32.try_into().unwrap();
    println!("index {index}");
}

// Example 6: parsing text into values.
fn demo_parse() {
    println!("-- parse --");

    let ok: Result<i32, _> = "42".parse();
    let bad: Result<i32, _> = "xi".parse();
    println!("{ok:?} {bad:?}");

    match "2.5".parse::<f64>() {
        Ok(v) => println!("float {v}"),
        Err(e) => println!("err {e}"),
    }

    // Each type has its own rules.
    println!("{:?} {:?}", "-5".parse::<u32>(), " 5".parse::<u32>());
    println!("{:?} {:?}", "true".parse::<bool>(), "x".parse::<char>());
    println!("{:?}", "1e3".parse::<f64>());

    // Parse a radix other than ten.
    println!("{:?} {:?}", i32::from_str_radix("ff", 16), u8::from_str_radix("101", 2));

    // A common pattern: parse with a default.
    let port: u16 = "not a port".parse().unwrap_or(8080);
    println!("port {port}");
}

fn main() {
    demo_arithmetic();
    demo_comparison();
    demo_bits();
    demo_as_casts();
    demo_checked_conversions();
    demo_parse();
}
