// 184. Any
//
// dyn Any is a type-erased value you can downcast. downcast_ref::<T>() returns
// Option<&T>. It is how error types recover a concrete type, and how some plugin systems
// store mixed values. Prefer enums when you know the set of types.
//
// Run: cargo run --bin 184_any

use std::any::Any;

fn show(value: &dyn Any) {
    if let Some(n) = value.downcast_ref::<i32>() {
        println!("i32 {n}");
    } else if let Some(s) = value.downcast_ref::<String>() {
        println!("string {s}");
    }
}

fn main() {
    show(&7i32);
    show(&String::from("hi"));
}
