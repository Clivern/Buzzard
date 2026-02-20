// 067. TryFrom and TryInto
//
// TryFrom is the fallible cousin of From. i32 to u8 can fail. try_into() needs a type on
// the left. The error type is associated on the trait. Use this instead of as when a
// too-large value should be reported, not truncated.
//
// Run: cargo run --bin 067_tryfrom

use std::convert::TryFrom;

fn main() {
    let ok = u8::try_from(12i32);
    let bad = u8::try_from(300i32);
    println!("{ok:?} {bad:?}");
    let n: Result<u8, _> = 40i32.try_into();
    println!("{n:?}");
}
