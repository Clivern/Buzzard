// 063. unwrap and expect
//
// unwrap returns the Ok/Some value or panics. expect is unwrap with a message you write.
// Use them when a None or Err would be a bug, not an expected failure. In examples they
// keep the code short. ok_or converts Option into Result.
//
// Run: cargo run --bin 063_unwrap

fn main() {
    let n: i32 = "12".parse().expect("digits");
    let m = Some(4).unwrap();
    println!("{n} {m}");
    let r: Result<i32, &str> = None.ok_or("missing");
    println!("{r:?}");
}
