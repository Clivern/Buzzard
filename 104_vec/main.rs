// 104. Vec
//
// Vec<T> is a growable array. vec![] builds one. push adds, pop removes from the end.
// len and capacity are separate: capacity is heap room already reserved. with_capacity
// avoids extra allocations. Indexing panics; get returns Option.
//
// Run: cargo run --bin 104_vec

fn main() {
    let mut v = Vec::with_capacity(2);
    v.push(1);
    v.push(2);
    v.push(3);
    println!("len {} cap {} {:?}", v.len(), v.capacity(), v.get(9));
    println!("{:?}", v.pop());
}
