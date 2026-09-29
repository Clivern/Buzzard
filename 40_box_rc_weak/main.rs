// 40. Box, Rc, and Weak
//
// Box<T> puts a T on the heap and owns it. Use it for recursive types (a node cannot
// contain itself inline, but it can contain a Box of itself), for trait objects
// (Box<dyn Trait>), and to move a large value around by moving only a pointer.
//
// Rc<T> is a reference-counted pointer for sharing one value between several owners on
// one thread. Rc::clone bumps a counter instead of copying the data; dropping an Rc
// lowers it; the value is freed when the count reaches zero. Rc gives shared (read-only)
// access; combine it with RefCell (lesson 41) to mutate. For threads, use Arc.
//
// Weak<T> is a non-owning handle to an Rc's value. It does not keep the value alive, and
// upgrade() returns None once the value is gone. Use Weak for back-pointers (child to
// parent) so a pair of Rcs does not form a cycle that never gets freed.
//
// Run: cargo run --bin 40_box_rc_weak

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Example 1: Box basics.
fn demo_box() {
    println!("-- Box --");

    let b = Box::new(5);
    println!("*b = {} b + 1 = {}", *b, *b + 1);

    let big = Box::new([0u8; 1024]);
    let moved = big; // only the pointer moves
    println!(
        "boxed array len {} Box size {} bytes",
        moved.len(),
        size_of::<Box<[u8; 1024]>>()
    );

    let s: Box<str> = "boxed str".into();
    let v: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    println!("{s} {v:?}");

    let unboxed: [u8; 1024] = *moved; // move out of a Box with *
    println!("unboxed first {}", unboxed[0]);
}

#[derive(Debug)]
struct Node {
    n: i32,
    next: Option<Box<Node>>,
}

fn list_from(items: &[i32]) -> Option<Box<Node>> {
    items.iter().rev().fold(None, |next, &n| Some(Box::new(Node { n, next })))
}

// Example 2: a recursive type built from Box.
fn demo_recursive() {
    println!("-- recursive Box --");

    let list = list_from(&[1, 2, 3]);
    let mut cur = list.as_deref();
    let mut out = Vec::new();
    while let Some(node) = cur {
        out.push(node.n);
        cur = node.next.as_deref();
    }

    println!("walked {out:?}");

    // struct Bad { n: i32, next: Option<Bad> }
    // error[E0072]: recursive type `Bad` has infinite size
}

// Example 3: Rc counts owners.
fn demo_rc() {
    println!("-- Rc --");

    let a = Rc::new(String::from("shared"));
    println!("count {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    {
        let c = Rc::clone(&a);
        println!("inside: {a} {b} {c} count {}", Rc::strong_count(&a));
    }

    println!("after scope count {}", Rc::strong_count(&a));
    drop(b);
    println!("after drop(b) count {}", Rc::strong_count(&a));
    println!("ptr_eq {}", Rc::ptr_eq(&a, &a.clone()));

    // Rc gives shared access only:
    // a.push_str("!"); // error[E0596]: cannot borrow data in an `Rc` as mutable
}

// Example 4: get_mut, make_mut, try_unwrap.
fn demo_rc_ownership() {
    println!("-- Rc get_mut / make_mut / try_unwrap --");

    let mut solo = Rc::new(vec![1]);
    Rc::get_mut(&mut solo).unwrap().push(2); // works: only one owner
    println!("get_mut while unique {solo:?}");

    let other = Rc::clone(&solo);
    println!("get_mut while shared {:?}", Rc::get_mut(&mut solo).is_some());
    Rc::make_mut(&mut solo).push(3); // clones the Vec because it is shared
    println!("make_mut cloned: solo {solo:?} other {other:?}");

    println!("try_unwrap shared {:?}", Rc::try_unwrap(Rc::clone(&other)).is_err());
    println!("try_unwrap unique {:?}", Rc::try_unwrap(solo));
}

// Example 5: shared ownership in a graph.
#[derive(Debug)]
struct Config {
    name: &'static str,
}

struct Service {
    id: u32,
    config: Rc<Config>,
}

fn demo_shared_graph() {
    println!("-- shared config --");

    let cfg = Rc::new(Config { name: "prod" });

    let services: Vec<Service> =
        (1..=3).map(|id| Service { id, config: Rc::clone(&cfg) }).collect();
    for s in &services {
        println!("service {} uses {}", s.id, s.config.name);
    }

    println!("config owners {}", Rc::strong_count(&cfg));
    drop(services);
    println!("after services dropped {}", Rc::strong_count(&cfg));
}

// Example 6: Weak references.
fn demo_weak() {
    println!("-- Weak --");

    let a = Rc::new(1);
    let w: Weak<i32> = Rc::downgrade(&a);
    println!(
        "upgrade {:?} strong {} weak {}",
        w.upgrade(),
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    drop(a);
    println!("after drop upgrade {:?}", w.upgrade());
}

#[derive(Debug)]
struct TreeNode {
    name: &'static str,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        println!("  drop {}", self.name);
    }
}

// Example 7: a tree with Weak parent pointers, so nothing leaks.
fn demo_tree() {
    println!("-- tree with Weak parent --");

    let leaf = Rc::new(TreeNode {
        name: "leaf",
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    {
        let root = Rc::new(TreeNode {
            name: "root",
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&root);

        let parent_name = leaf.parent.borrow().upgrade().map(|p| p.name);
        println!(
            "  leaf's parent {parent_name:?}, root has {} child",
            root.children.borrow().len()
        );
        println!(
            "  root strong {} weak {}",
            Rc::strong_count(&root),
            Rc::weak_count(&root)
        );
    }

    // root was dropped (its only strong owner went away); the Weak did not keep it alive.
    println!("  leaf's parent now {:?}", leaf.parent.borrow().upgrade().map(|p| p.name));
}

fn main() {
    demo_box();
    demo_recursive();
    demo_rc();
    demo_rc_ownership();
    demo_shared_graph();
    demo_weak();
    demo_tree();
}
