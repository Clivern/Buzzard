// 165. static vs const
//
// const is inlined. static has a fixed address. mut static requires unsafe to read or
// write because of data races. Prefer OnceLock or Mutex for mutable globals. Immutable
// static of a Sync type is fine, such as a string literal.
//
// Run: cargo run --bin 165_static_vs_const

const MAX: i32 = 10;
static NAME: &str = "learn";

fn main() {
    println!("{MAX} {NAME} {:p}", NAME as *const str);
}
