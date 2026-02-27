// 088. Associated types
//
// A trait can name a type with type Item;. Implementors pick the concrete type.
// Iterator::Item is the classic example. Associated types are for one type per impl;
// generic traits (From<T>) are for many impls on the same type.
//
// Run: cargo run --bin 088_associated_types

trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

impl<T> Container for Vec<T> {
    type Item = T;
    fn first(&self) -> Option<&Self::Item> {
        self.as_slice().first()
    }
}

fn main() {
    println!("{:?}", Container::first(&vec![10, 20]));
}
