// 03. Scalar types: integers, floats, bools, and chars
//
// Integers have a signedness and a width: i8..i128, u8..u128, and isize/usize, which
// are pointer-sized and used for indexing. In debug builds overflow panics; in release it
// wraps. Methods such as checked_add, wrapping_add, and saturating_add pick a policy
// explicitly, and the Wrapping and Saturating types make the operators do it.
//
// f64 is the default float. Floats follow IEEE 754: 0.1 + 0.2 is not exactly 0.3, and
// NaN is not equal to itself. bool is true or false, and && and || short circuit. char
// is a four-byte Unicode scalar value, written in single quotes. It is not a byte.
//
// NonZeroU32 and friends are integers that cannot be zero. Option<NonZeroU32> is the
// same size as u32, because None can use the zero bit pattern.
//
// Run: cargo run --bin 03_scalars

use std::num::{NonZeroU32, Saturating, Wrapping};

// Example 1: literals, suffixes, and separators.
fn demo_literals() {
    println!("-- literals --");

    let a: i32 = -3;
    let b = 10u8;
    let big = 1_000_000_u64;
    let hex = 0xff;
    let oct = 0o17;
    let bin = 0b1010_1010;
    let byte = b'A';
    let c = 2.5_f64;
    println!("{a} {b} {big} {hex} {oct} {bin} {byte} {c}");
}

// Example 2: ranges and sizes of the integer types.
fn demo_integer_ranges() {
    println!("-- integer ranges --");

    println!("i8 {}..={}", i8::MIN, i8::MAX);
    println!("u8 {}..={}", u8::MIN, u8::MAX);
    println!("i32 {}..={}", i32::MIN, i32::MAX);
    println!("u64 max {}", u64::MAX);
    println!("i128 bytes {} usize bytes {}", size_of::<i128>(), size_of::<usize>());
    println!("bits in u16 {}", u16::BITS);
}

// Example 3: choosing an overflow policy.
fn demo_overflow() {
    println!("-- overflow --");

    let n: u8 = 250;
    println!("checked {:?} {:?}", n.checked_add(10), n.checked_add(5));
    println!("wrapping {}", n.wrapping_add(10));
    println!("saturating {}", n.saturating_add(10));

    let (value, overflowed) = n.overflowing_add(10);
    println!("overflowing {value} {overflowed}");

    // In a debug build this line would panic: attempt to add with overflow.
    // let boom = n + 10;

    // Wrapping and Saturating apply the policy to the operators themselves.
    let w = Wrapping(250u8) + Wrapping(10u8);
    let s = Saturating(250u8) + Saturating(10u8);
    println!("Wrapping {} Saturating {}", w.0, s.0);

    // A tiny checksum is a classic use of wrapping arithmetic.
    let sum = b"hello".iter().fold(Wrapping(0u8), |acc, &b| acc + Wrapping(b));
    println!("checksum {}", sum.0);
}

// Example 4: useful integer methods.
fn demo_integer_methods() {
    println!("-- integer methods --");

    let n: i32 = -17;
    println!("abs {} pow {} signum {}", n.abs(), 2i32.pow(10), n.signum());
    println!(
        "div {} rem {} euclid {} {}",
        n / 5,
        n % 5,
        n.div_euclid(5),
        n.rem_euclid(5)
    );
    println!("min {} max {} clamp {}", n.min(3), n.max(3), 120.clamp(0, 100));
    println!("abs_diff {}", 3u8.abs_diff(10));

    let bits: u8 = 0b0110_1000;
    println!(
        "ones {} leading {} trailing {}",
        bits.count_ones(),
        bits.leading_zeros(),
        bits.trailing_zeros()
    );
    println!(
        "power of two? {} next {}",
        64u32.is_power_of_two(),
        100u32.next_power_of_two()
    );
}

// Example 5: floats and their traps.
fn demo_floats() {
    println!("-- floats --");

    let x = 7.0 / 2.0;
    let y: f32 = 1.0 / 3.0;
    println!("f64 {x} f32 {y}");
    println!("0.1 + 0.2 == 0.3? {}", (0.1 + 0.2) == 0.3);
    println!("close enough? {}", ((0.1_f64 + 0.2) - 0.3).abs() < f64::EPSILON);

    let nan = f64::NAN;
    let other_nan = f64::NAN;
    println!("nan {} nan == nan? {}", nan.is_nan(), nan == other_nan);
    println!("inf {} -1/0 {}", f64::INFINITY.is_infinite(), -1.0 / 0.0);

    let v = 2.7_f64;
    println!(
        "floor {} ceil {} round {} trunc {} fract {:.1}",
        v.floor(),
        v.ceil(),
        v.round(),
        v.trunc(),
        v.fract()
    );
    println!(
        "sqrt {:.3} powi {} max {}",
        2f64.sqrt(),
        1.5f64.powi(2),
        1.0f64.max(f64::NAN)
    );

    // Floats are not Ord because of NaN, so sort() does not compile on them.
    // total_cmp gives a total order, putting NaN last.
    let mut vals = [2.5, -1.0, f64::NAN, 0.0];
    vals.sort_by(|a, b| a.total_cmp(b));
    println!("sorted {vals:?}");
}

// Example 6: bools and short circuiting.
fn demo_bools() {
    println!("-- bools --");

    let ready = true;
    let empty = false;
    println!(
        "and {} or {} not {} xor {}",
        ready && empty,
        ready || empty,
        !empty,
        ready ^ empty
    );

    // The right side of && only runs if the left side is true.
    let n = 0;
    if n != 0 && 10 / n > 1 {
        println!("skipped because n is 0");
    } else {
        println!("short circuit kept us safe");
    }

    // There is no truthiness: `if n {}` with an integer is a type error.
    println!("as int {} {}", true as i32, false as u8);
    println!("then {:?} {:?}", ready.then(|| "yes"), empty.then_some("no"));
}

// Example 7: chars are Unicode scalar values, not bytes.
fn demo_chars() {
    println!("-- chars --");

    let crab: char = '🦀';
    println!("crab {crab} U+{:X} bytes {}", crab as u32, crab.len_utf8());
    println!("char size {}", size_of::<char>());
    println!("escapes [{}] [{}] [{}]", '\t', '\'', '\u{2603}');
    println!(
        "alpha {} digit {} upper {}",
        'a'.is_alphabetic(),
        '7'.is_ascii_digit(),
        'q'.to_ascii_uppercase()
    );
    println!("to_digit {:?} from_digit {:?}", '7'.to_digit(10), char::from_digit(9, 10));
    println!("from u32 {:?} {:?}", char::from_u32(0x41), char::from_u32(0xD800));

    let s = "Go → 世";
    println!("bytes {} chars {}", s.len(), s.chars().count());

    for c in s.chars() {
        println!("  {c} U+{:04X}", c as u32);
    }
}

// Example 8: NonZero integers.
fn demo_nonzero() {
    println!("-- nonzero --");

    let n = NonZeroU32::new(4).unwrap();
    println!("{} {:?}", n.get(), NonZeroU32::new(0));
    println!(
        "sizes u32 {} Option<NonZeroU32> {} Option<u32> {}",
        size_of::<u32>(),
        size_of::<Option<NonZeroU32>>(),
        size_of::<Option<u32>>()
    );

    // Dividing by a NonZero cannot panic, so the compiler needs no zero check.
    println!("100 / n = {}", 100 / n);
}

fn main() {
    demo_literals();
    demo_integer_ranges();
    demo_overflow();
    demo_integer_methods();
    demo_floats();
    demo_bools();
    demo_chars();
    demo_nonzero();
}
