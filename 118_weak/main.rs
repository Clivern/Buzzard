// 118. Weak
//
// Weak<T> is a non-owning Rc. It does not keep T alive. upgrade() returns Option<Rc<T>>.
// Use Weak to break cycles: a parent holds Rc children, a child holds Weak parent.
// Cycles of Rc leak; Weak does not.
//
// Run: cargo run --bin 118_weak

use std::rc::{Rc, Weak};

fn main() {
    let a = Rc::new(1);
    let w: Weak<i32> = Rc::downgrade(&a);
    println!("{:?}", w.upgrade());
    drop(a);
    println!("{:?}", w.upgrade());
}
