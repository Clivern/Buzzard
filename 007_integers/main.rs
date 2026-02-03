// 007. Integer overflow and sizes
//
// Integer types differ in range. i8 goes from -128 to 127. usize is the pointer width,
// used for indexing. In debug builds, overflow panics. In release builds it wraps.
// wrapping_add, saturating_add, and checked_add choose the policy yourself. checked_add
// returns None on overflow.
//
// Run: cargo run --bin 007_integers

fn main() {
    let n: u8 = 250;
    println!("wrapping {}", n.wrapping_add(10));
    println!("saturating {}", n.saturating_add(10));
    println!("checked {:?}", n.checked_add(10));
    println!("usize bytes {}", std::mem::size_of::<usize>());
}
