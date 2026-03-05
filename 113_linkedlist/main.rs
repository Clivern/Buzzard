// 113. LinkedList
//
// LinkedList is a doubly linked list. It is rarely the right default: Vec and VecDeque
// are faster in almost every case because of cache locality. It can split and append
// lists in O(1). Prefer Vec unless you measured a need.
//
// Run: cargo run --bin 113_linkedlist

use std::collections::LinkedList;

fn main() {
    let mut a = LinkedList::from([1, 2]);
    let mut b = LinkedList::from([3, 4]);
    a.append(&mut b);
    println!("{a:?}");
}
