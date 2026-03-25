// 195. NonZero types
//
// NonZeroU32 is a u32 that cannot be zero. Option<NonZeroU32> is the same size as u32
// because of null-pointer optimization. new() returns Option. get() returns the integer.
// Use them for IDs and divisor values where zero is invalid.
//
//
// get() is a plain integer again. The type system only helps while the value stays
// NonZero.
//
// Run: cargo run --bin 195_nonzero

use std::num::NonZeroU32;

fn main() {
    let n = NonZeroU32::new(4).unwrap();
    println!("{} {:?}", n.get(), NonZeroU32::new(0));
    println!(
        "sizes {} {}",
        std::mem::size_of::<u32>(),
        std::mem::size_of::<Option<NonZeroU32>>()
    );
    println!("bits {}", n.get().count_ones());
}
