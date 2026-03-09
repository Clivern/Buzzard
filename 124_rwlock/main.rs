// 124. RwLock
//
// RwLock allows many readers or one writer. read() and write() return guards. A writer
// waits for readers. Prefer Mutex if critical sections are short or writes are frequent;
// the extra complexity of RwLock pays off for read-heavy data.
//
// Run: cargo run --bin 124_rwlock

use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("{} {}", *r1, *r2);
    }
    *lock.write().unwrap() += 1;
    println!("{}", *lock.read().unwrap());
}
