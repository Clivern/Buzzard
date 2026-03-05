// 111. VecDeque
//
// VecDeque is a double-ended queue: push_front, push_back, pop_front, pop_back are O(1).
// It is a ring buffer. Use it for queues, BFS, and sliding windows. Indexing works; 0 is
// the front.
//
// Run: cargo run --bin 111_vecdeque

use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_front(0);
    println!("{:?} {:?}", q.pop_front(), q);
}
