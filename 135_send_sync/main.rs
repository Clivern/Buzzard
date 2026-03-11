// 135. Send and Sync
//
// Send means you can move a value to another thread. Sync means you can share a &T
// across threads (T is Sync iff &T is Send). Rc is neither; Arc is both. Mutex makes T:
// Send into Mutex<T>: Sync. The compiler implements them automatically for most types.
// Unsafe impl is rare and easy to get wrong.
//
// Run: cargo run --bin 135_send_sync

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

fn main() {
    is_send::<i32>();
    is_sync::<i32>();
    is_send::<std::sync::Arc<i32>>();
    println!("ok");
}
