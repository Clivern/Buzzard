// 032. Ranges
//
// a..b is start inclusive, end exclusive. a..=b includes the end. ..b is an exclusive
// end with an open start, used in slicing. 1.. is an open-ended iterator; take() stops
// it. Range is a real type; you can store it and pass it around.
//
// Run: cargo run --bin 032_range

fn main() {
    let r = 1..=3;
    println!("{:?}", r.clone().collect::<Vec<_>>());
    let a = [10, 20, 30, 40];
    println!("{:?}", &a[1..3]);
    println!("{:?}", (10..).take(3).collect::<Vec<_>>());
}
