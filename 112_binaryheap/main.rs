// 112. BinaryHeap
//
// BinaryHeap is a max-heap. peek is the largest. pop removes it. For a min-heap, store
// Reverse(n). Use it for priority queues and Dijkstra. It is a Vec under the hood;
// into_sorted_vec consumes it into a sorted Vec.
//
// Run: cargo run --bin 112_binaryheap

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut h = BinaryHeap::from([3, 1, 4]);
    println!("{:?}", h.peek());
    let mut min = BinaryHeap::new();
    min.push(Reverse(3));
    min.push(Reverse(1));
    println!("{:?}", min.pop());
}
