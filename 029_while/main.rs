// 029. while
//
// while condition { } repeats as long as the condition is true. The condition must be a
// bool. while is for when you do not know the count in advance. For a known range, for
// is clearer.
//
// Run: cargo run --bin 029_while

fn main() {
    let mut n = 0;
    while n < 3 {
        println!("n {n}");
        n += 1;
    }
}
