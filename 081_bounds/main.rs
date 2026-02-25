// 081. Trait bounds
//
// T: Display + Clone means T must implement both. Bounds can go on the function, the
// impl, or a where clause. where is easier to read when bounds are long or when you
// bound associated types. Without a bound, T is opaque: you can only move it and drop
// it.
//
// Run: cargo run --bin 081_bounds

fn dump<T>(value: T)
where
    T: std::fmt::Debug + Clone,
{
    let copy = value.clone();
    println!("{copy:?}");
}

fn main() {
    dump(vec![1, 2, 3]);
}
