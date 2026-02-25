// 082. impl Trait
//
// impl Trait in argument position is a generic: the caller picks the type. impl Trait in
// return position hides the concrete type, so you can return a closure or a long
// iterator type without writing it. Return-position impl Trait can only name one
// concrete type; use Box<dyn Trait> for several.
//
// Run: cargo run --bin 082_impl_trait

fn add_one(x: impl Into<i32>) -> i32 {
    x.into() + 1
}

fn odds() -> impl Iterator<Item = i32> {
    (0..10).filter(|n| n % 2 == 1)
}

fn main() {
    println!("{}", add_one(3u8));
    println!("{:?}", odds().collect::<Vec<_>>());
}
