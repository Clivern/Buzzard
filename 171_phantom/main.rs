// 171. PhantomData
//
// PhantomData<T> is a zero-sized marker that tells dropck and variance you logically own
// or borrow T even though you have no field of type T. It shows up in typed IDs, in
// unsafe wrappers, and in generic structs that store raw pointers.
//
// Run: cargo run --bin 171_phantom

use std::marker::PhantomData;

struct Id<T> {
    n: u64,
    _ty: PhantomData<T>,
}

struct User;
struct Team;

fn main() {
    let u = Id::<User> { n: 1, _ty: PhantomData };
    let _t = Id::<Team> { n: 1, _ty: PhantomData };
    println!("{}", u.n);
}
