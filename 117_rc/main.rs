// 117. Rc
//
// Rc<T> is a reference-counted pointer for single-threaded sharing. clone bumps the
// count; drop lowers it. When the count hits zero, T is dropped. Rc::get_mut works only
// when the count is 1. For threads, use Arc.
//
// Run: cargo run --bin 117_rc

use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("{} count {}", a, Rc::strong_count(&a));
    drop(b);
    println!("count {}", Rc::strong_count(&a));
}
