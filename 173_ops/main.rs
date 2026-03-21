// 173. Operator traits
//
// std::ops traits overload operators: Add, Sub, Index, Not, and so on. impl Add for
// Watts so + works. Output is an associated type. You cannot invent new operators. Index
// returns a reference; IndexMut is the mutable side.
//
// Run: cargo run --bin 173_ops

use std::ops::Add;

struct Watts(u32);

impl Add for Watts {
    type Output = Watts;
    fn add(self, rhs: Watts) -> Watts {
        Watts(self.0 + rhs.0)
    }
}

fn main() {
    let w = Watts(40) + Watts(20);
    println!("{}", w.0);
}
