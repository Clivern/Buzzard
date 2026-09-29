// 33. Closures and Fn, FnMut, FnOnce
//
// A closure is an anonymous function that can capture variables from where it was
// written: |n| n + factor. The compiler infers parameter and return types when it can.
// Each closure has its own unique, unnameable type, so you pass closures around as
// generics (impl Fn...) or boxed trait objects (Box<dyn Fn...>).
//
// Closures capture as little as they need: by shared reference, by mutable reference,
// or by value. move forces capture by value, which you need when the closure outlives
// the current function, such as when it runs on another thread or is returned.
//
// How a closure uses its captures decides which traits it implements. Fn: only reads,
// callable many times. FnMut: mutates captures, callable many times. FnOnce: may
// consume a capture, callable once. Every Fn is also FnMut, and every FnMut is FnOnce.
//
// Run: cargo run --bin 33_closures

// Example 1: syntax and inference.
fn demo_syntax() {
    println!("-- syntax --");

    let add = |a, b| a + b;
    let add_typed = |a: i32, b: i32| -> i32 { a + b };
    let greet = |name: &str| format!("hi {name}");
    let answer = || 42;
    println!("{} {} {} {}", add(2, 3), add_typed(4, 5), greet("Ada"), answer());

    // The first call fixes the types:
    // add(1.5, 2.0); // error[E0308]: arguments to this function are incorrect
}

// Example 2: capturing by reference, mutable reference, and value.
fn demo_capture() {
    println!("-- capture --");

    let factor = 10;
    let scale = |n| n * factor; // borrows factor
    println!("scale {} factor still usable {factor}", scale(4));

    let mut count = 0;
    let mut bump = || {
        count += 1; // mutably borrows count
        count
    };

    println!("bump {} {}", bump(), bump());
    println!("count after {count}");

    let s = String::from("owned");
    let consume = move || s.len(); // s moved into the closure
    println!("len {}", consume());

    // println!("{s}"); // error[E0382]: borrow of moved value: `s`
}

// Example 3: move with Copy types, and cloning before move.
fn demo_move() {
    println!("-- move --");

    let n = 5;
    let f = move || n + 1; // i32 is Copy, so n stays usable
    println!("{} and n is still {n}", f());

    let name = String::from("shared");
    let name_for_closure = name.clone();
    let shout = move || name_for_closure.to_uppercase();
    println!("{} / original {name}", shout());
}

fn apply<F: Fn(i32) -> i32>(n: i32, f: F) -> i32 {
    f(n)
}

fn apply_twice<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn run_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn double(n: i32) -> i32 {
    n * 2
}

// Example 4: Fn, FnMut, FnOnce as bounds.
fn demo_fn_traits() {
    println!("-- Fn / FnMut / FnOnce --");

    println!("{} {}", apply(3, |n| n + 1), apply(3, double));

    let mut log = Vec::new();
    apply_twice(|| log.push("tick"));
    println!("FnMut ran twice: {log:?}");

    let msg = String::from("consumed");
    let give_away = move || msg; // returns the capture, so it is FnOnce
    println!("FnOnce: {}", run_once(give_away));

    // give_away(); // error[E0382]: use of moved value: `give_away`
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn make_counter() -> impl FnMut() -> u32 {
    let mut c = 0;
    move || {
        c += 1;
        c
    }
}

fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

// Example 5: returning closures.
fn demo_returning() {
    println!("-- returning closures --");

    let add5 = make_adder(5);
    println!("add5(1) = {}", add5(1));

    let mut next = make_counter();
    println!("counter {} {} {}", next(), next(), next());

    let inc_then_double = compose(|x: i32| x + 1, |x| x * 2);
    println!("compose(3) = {}", inc_then_double(3));
}

// Example 6: storing closures in structs and collections.
struct Button {
    label: String,
    on_click: Box<dyn Fn(&str) -> String>,
}

struct Cache<F: Fn(u64) -> u64> {
    f: F,
    memo: std::collections::HashMap<u64, u64>,
}

impl<F: Fn(u64) -> u64> Cache<F> {
    fn get(&mut self, n: u64) -> u64 {
        if let Some(v) = self.memo.get(&n) {
            return *v;
        }

        let v = (self.f)(n);
        self.memo.insert(n, v);
        v
    }
}

fn demo_storing() {
    println!("-- storing closures --");

    let buttons = vec![
        Button { label: "ok".into(), on_click: Box::new(|l| format!("{l} clicked")) },
        Button { label: "cancel".into(), on_click: Box::new(|l| l.to_uppercase()) },
    ];

    for b in &buttons {
        println!("{}", (b.on_click)(&b.label));
    }

    let mut slow_square = Cache { f: |n| n * n, memo: Default::default() };
    println!(
        "{} {} memo size {}",
        slow_square.get(12),
        slow_square.get(12),
        slow_square.memo.len()
    );
}

// Example 7: closures with iterator adapters and sort_by_key.
fn demo_with_iterators() {
    println!("-- with iterators --");

    let threshold = 3;
    let v: Vec<_> = (1..=6).filter(|n| *n > threshold).map(|n| n * 10).collect();
    println!("{v:?}");

    let mut people = vec![("cy", 41), ("ann", 30), ("ben", 35)];
    people.sort_by_key(|&(_, age)| age);
    println!("{people:?}");

    let mut total = 0;
    people.iter().for_each(|(_, age)| total += age);
    println!("total age {total}");

    // Function names and methods work wherever a closure is expected.
    let lens: Vec<usize> = ["a", "bcd"].iter().map(|s| s.len()).collect();
    let upper: Vec<String> = ["x", "y"].iter().map(|s| s.to_uppercase()).collect();
    let somes: Vec<Option<i32>> = [1, 2].into_iter().map(Some).collect();
    println!("{lens:?} {upper:?} {somes:?}");
}

fn main() {
    demo_syntax();
    demo_capture();
    demo_move();
    demo_fn_traits();
    demo_returning();
    demo_storing();
    demo_with_iterators();
}
