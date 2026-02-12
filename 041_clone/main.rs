// 041. clone
//
// clone duplicates a value by copying its heap data. It is explicit, so you see the
// cost. Clone is a trait; #[derive(Clone)] works when every field is Clone. clone on a
// reference clones the pointed-to value, not the reference.
//
// Run: cargo run --bin 041_clone

fn main() {
    let a = String::from("hello");
    let b = a.clone();
    println!("{a} {b}");
}
