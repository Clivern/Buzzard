// 116. Box
//
// Box<T> is a heap allocation. Box::new(v) moves v to the heap. Use it for recursive
// types (a cons list cannot store List inline), large values you want to move cheaply,
// or trait objects. Dereference with * or method calls.
//
// Run: cargo run --bin 116_box_ptr

fn main() {
    let b = Box::new(5);
    println!("{}", *b);
    let list = Node {
        n: 1,
        next: Some(Box::new(Node { n: 2, next: None })),
    };
    println!("{}", list.next.unwrap().n);
}

struct Node {
    n: i32,
    next: Option<Box<Node>>,
}
