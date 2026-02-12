// 042. Copy
//
// Copy is a marker trait for types that are safe to duplicate with a memcpy: integers,
// bool, char, shared references, and structs of Copy fields. A type cannot be Copy if it
// implements Drop. Moving a Copy type still leaves the old name usable.
//
// Run: cargo run --bin 042_copy

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("{a:?} {b:?}");
}
