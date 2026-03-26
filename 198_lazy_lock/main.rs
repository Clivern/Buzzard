// 198. LazyLock
//
// LazyLock<T> is a OnceLock that initializes on first deref. It can be static. The
// closure runs at most once. It is the std replacement for the lazy_static crate in most
// cases. Prefer it for read-mostly global config.
//
// Run: cargo run --bin 198_lazy_lock

use std::sync::LazyLock;

static NUMS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 2, 3]);

fn main() {
    println!("{} {}", NUMS.len(), NUMS[1]);
}
