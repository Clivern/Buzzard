// 086. Derive
//
// #[derive(Clone, Debug)] asks the compiler to generate those impls. Built-in derives
// include Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash. Third-party
// derives (serde) need a crate. Derive fails if a field does not implement the trait.
//
// Run: cargo run --bin 086_derive

#[derive(Clone, Debug, PartialEq)]
struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let p = Pair { a: 1, b: 2 };
    let q = p.clone();
    println!("{:?} {}", q, p == q);
}
