// 201. thread_local
//
// thread_local! gives each thread its own value. with(|v| ...) borrows it. It is useful
// for caches and scratch buffers that must not be shared. Values are dropped when the
// thread exits. Do not use it as a hidden global for business logic.
//
// Run: cargo run --bin 201_thread_local

use std::cell::Cell;

thread_local! {
    static COUNT: Cell<u32> = const { Cell::new(0) };
}

fn main() {
    COUNT.with(|c| c.set(c.get() + 1));
    COUNT.with(|c| println!("{}", c.get()));
}
