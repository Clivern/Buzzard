// 166. matches! and if matches
//
// matches!(value, pattern) is a bool. It is handy in asserts and filters. inspect the
// same patterns you would in match, including guards.
//
//
// matches! is allowed in a while or if condition. It reads better than a full match when
// you only need a boolean.
//
// Run: cargo run --bin 166_matches

fn main() {
    let n = Some(3);
    println!("{}", matches!(n, Some(x) if x > 0));
    let v = vec![Ok(1), Err("e"), Ok(2)];
    let oks: Vec<_> = v.into_iter().filter(|r| matches!(r, Ok(_))).collect();
    println!("{oks:?}");
    if matches!(n, Some(3)) {
        println!("three");
    }
}
