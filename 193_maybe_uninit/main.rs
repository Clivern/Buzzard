// 193. MaybeUninit
//
// MaybeUninit<T> is uninitialized memory for T. assume_init is unsafe: you must have
// written a valid T first. Arrays of MaybeUninit let you fill a buffer item by item.
// Prefer Vec and array::from_fn when they work; MaybeUninit is for avoiding double drops
// or extra Default.
//
// Run: cargo run --bin 193_maybe_uninit

use std::mem::MaybeUninit;

fn main() {
    let mut slot: MaybeUninit<i32> = MaybeUninit::uninit();
    slot.write(7);
    let n = unsafe { slot.assume_init() };
    println!("{n}");
}
