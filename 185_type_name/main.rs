// 185. type_name
//
// std::any::type_name::<T>() is a compiler-provided string. It is for debug logs, not
// stable identity: the text can change between versions. TypeId is the runtime identity
// used by Any.
//
// Run: cargo run --bin 185_type_name

use std::any::{TypeId, type_name};

fn main() {
    println!("{} {:?}", type_name::<Vec<i32>>(), TypeId::of::<Vec<i32>>());
}
