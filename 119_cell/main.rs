// 119. Cell
//
// Cell<T> is interior mutability for Copy types. get copies out, set replaces. There are
// no runtime borrow checks. It is not Sync. Use it when you need to mutate through a
// shared &self and T is Copy.
//
// Run: cargo run --bin 119_cell

use std::cell::Cell;

struct Flag {
    n: Cell<i32>,
}

fn main() {
    let f = Flag { n: Cell::new(0) };
    f.n.set(f.n.get() + 1);
    println!("{}", f.n.get());
}
