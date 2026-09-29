// 11. Clone and Copy
//
// Assigning or passing a value normally moves it. Two traits change that:
//
// - Copy: an implicit bitwise duplicate. The old name stays usable after `let b = a`.
//   Only for types where a memcpy is a correct copy: integers, floats, bool, char,
//   shared refs, and tuples/arrays/structs made only of Copy parts.
// - Clone: an explicit `.clone()` that may do real work (allocate, bump a refcount).
//
// Every Copy type must also be Clone. A type cannot be Copy if it implements Drop or
// owns heap data (String, Vec, Box).
//
// Run: cargo run --bin 11_clone_copy

use std::rc::Rc;

// Example 1: built-in Copy types stay usable after assignment.
fn demo_builtin_copy() {
    println!("-- builtin copy --");

    let a = 5;
    let b = a;
    let c = 'x';
    let d = c;
    let t = (1, true, 2.5);
    let u = t;
    let arr = [1u8; 4];
    let arr2 = arr;
    println!("{a} {b} {c} {d} {t:?} {u:?} {arr:?} {arr2:?}");
}

// Example 2: a struct made of Copy fields can derive Copy.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn shift(mut p: Point) -> Point {
    p.x += 10;
    p
}

fn demo_derive_copy() {
    println!("-- derive copy --");

    let a = Point { x: 1, y: 2 };
    let b = shift(a); // a is copied into shift, not moved
    println!("a {a:?} b {b:?} equal? {}", a == b);
}

// Example 3: String is Clone but not Copy. Assignment moves; clone duplicates the heap
// buffer.
fn demo_clone_heap() {
    println!("-- clone heap --");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    let s3 = s1; // move: s1 is no longer usable
    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
    println!("{s2} {s3}");

    let v = vec![vec![1, 2], vec![3]];
    let mut w = v.clone(); // deep: the inner Vecs are cloned too
    w[0].push(99);
    println!("v {v:?} w {w:?}");
}

// Example 4: a struct with a String field can derive Clone, but not Copy.
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
}

// #[derive(Clone, Copy)]
// struct Bad { name: String }
// error[E0204]: the trait `Copy` cannot be implemented for this type

fn demo_clone_struct() {
    println!("-- clone struct --");

    let u1 = User { name: "ana".into(), age: 30 };
    let mut u2 = u1.clone();
    u2.name.push_str("_copy");
    u2.age += 1;
    println!("{u1:?} {u2:?}");
}

// Example 5: Clone does not have to be deep. Rc::clone only bumps a counter, and both
// handles point at the same data.
fn demo_cheap_clone() {
    println!("-- cheap clone --");

    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("{a} {b} count {} same? {}", Rc::strong_count(&a), Rc::ptr_eq(&a, &b));
}

// Example 6: a hand-written Clone, for when derive does the wrong thing.
#[derive(Debug)]
struct Counter {
    id: u32,
    hits: u32,
}

impl Clone for Counter {
    // A clone keeps the id but starts with zero hits.
    fn clone(&self) -> Self {
        Counter { id: self.id, hits: 0 }
    }
}

fn demo_manual_clone() {
    println!("-- manual clone --");

    let a = Counter { id: 7, hits: 42 };
    let b = a.clone();
    println!("{a:?} {b:?} hits {} -> {}", a.hits, b.hits);
}

// Example 7: Drop and Copy do not mix. A Drop type runs cleanup once, so silent
// duplicates would run it twice.
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

// impl Copy for Guard {}
// error[E0184]: the trait `Copy` cannot be implemented for this type; the type has a
// destructor

fn demo_drop_not_copy() {
    println!("-- drop not copy --");

    let g = Guard("guard");
    let moved = g; // a move, so drop runs exactly once at the end of scope
    let _ = &moved;
}

// Example 8: copied vs cloned on iterators turn &T into T.
fn demo_iter_copied_cloned() {
    println!("-- copied / cloned --");

    let nums = [3, 1, 2];
    let owned: Vec<i32> = nums.iter().copied().collect();
    let words = vec![String::from("a"), String::from("b")];
    let owned_words: Vec<String> = words.iter().cloned().collect();
    println!("{owned:?} {owned_words:?} (words still here: {words:?})");
}

fn main() {
    demo_builtin_copy();
    demo_derive_copy();
    demo_clone_heap();
    demo_clone_struct();
    demo_cheap_clone();
    demo_manual_clone();
    demo_drop_not_copy();
    demo_iter_copied_cloned();
}
