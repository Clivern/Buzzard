// 031. for loops
//
// for x in iterable walks IntoIterator. for x in 0..3 is 0, 1, 2. 0..=3 includes 3. for
// x in &v borrows. for x in &mut v mutably borrows. for x in v consumes and moves each
// element. enumerate() pairs an index with each item.
//
// Run: cargo run --bin 031_for

fn main() {
    for i in 0..3 {
        println!("count {i}");
    }
    let v = vec!["a", "b"];
    for (i, item) in v.iter().enumerate() {
        println!("{i} {item}");
    }
}
