// 120. RefCell
//
// RefCell<T> is interior mutability with runtime borrow checking. borrow() is &T,
// borrow_mut() is &mut T. A second borrow_mut panics. Use it when the compiler cannot
// see that borrows do not overlap, still single-threaded.
//
// Run: cargo run --bin 120_refcell

use std::cell::RefCell;

fn main() {
    let v = RefCell::new(vec![1]);
    v.borrow_mut().push(2);
    println!("{:?}", v.borrow());
}
