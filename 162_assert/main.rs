// 162. assert
//
// assert! panics if the condition is false. assert_eq! prints both sides. assert_ne! is
// the opposite. debug_assert! is stripped in release unless you enable debug assertions.
// Use asserts for internal invariants, Result for expected failures.
//
// Run: cargo run --bin 162_assert

fn main() {
    assert!(2 + 2 == 4);
    assert_eq!(3, 1 + 2);
    debug_assert!(true);
    println!("ok");
}
