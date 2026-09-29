// 25. Generics and trait bounds
//
// A type parameter in angle brackets, <T>, stands for a type the caller chooses.
// Functions, structs, enums, and impls can all be generic. The compiler monomorphizes:
// each concrete type used gets its own compiled copy, so generic code is as fast as
// hand-written code.
//
// Without a bound, a T is opaque: you can move it and drop it, nothing more. A bound
// such as T: Clone + Debug says what T must support, and unlocks those methods. Bounds
// can go inline, or in a where clause when they get long. Turbofish ::<T> names the
// type explicitly when inference cannot.
//
// Run: cargo run --bin 25_generics

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

fn first<T: Clone>(items: &[T]) -> Option<T> {
    items.first().cloned()
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &i in items {
        if i > best {
            best = i;
        }
    }

    best
}

// Example 1: generic functions and turbofish.
fn demo_functions() {
    println!("-- generic functions --");

    println!("{:?} {:?}", first(&["a", "b"]), first::<i32>(&[]));
    println!(
        "largest {} {} {}",
        largest(&[3, 7, 2]),
        largest(&[1.5, 0.2]),
        largest(&['x', 'b'])
    );

    let parsed = "5".parse::<u8>();
    let collected = (1..4).collect::<Vec<_>>();
    println!("{parsed:?} {collected:?}");

    // fn bad<T>(a: T, b: T) -> bool { a == b }
    // error[E0369]: binary operation `==` cannot be applied to type `T`
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Display> Point<T> {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

// Methods only for one concrete type.
impl Point<f64> {
    fn dist(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Debug)]
struct Pair<A, B> {
    a: A,
    b: B,
}

impl<A, B> Pair<A, B> {
    fn swap(self) -> Pair<B, A> {
        Pair { a: self.b, b: self.a }
    }
}

// Example 2: generic structs and impls.
fn demo_structs() {
    println!("-- generic structs --");

    Point { x: 1, y: 2 }.show();
    Point { x: "left", y: "right" }.show();

    let p = Point { x: 3.0, y: 4.0 };
    println!("dist {}", p.dist());

    // Point { x: 3, y: 4 }.dist(); // error[E0599]: no method named `dist` for Point<i32>
    // let mixed = Point { x: 1, y: 2.0 }; // error[E0308]: mismatched types
    let pair = Pair { a: 1, b: "one" };
    println!("{pair:?}");

    let swapped = pair.swap(); // swap takes self, so pair is moved here
    println!("swapped {swapped:?} a={} b={}", swapped.a, swapped.b);
}

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Debug, R: Debug> Either<L, R> {
    fn describe(&self) -> String {
        match self {
            Either::Left(l) => format!("left {l:?}"),
            Either::Right(r) => format!("right {r:?}"),
        }
    }
}

fn parse_or_keep(s: &str) -> Either<i32, &str> {
    s.parse().map(Either::Left).unwrap_or(Either::Right(s))
}

// Example 3: generic enums, like Option and Result.
fn demo_enums() {
    println!("-- generic enums --");

    for s in ["12", "twelve"] {
        println!("{}", parse_or_keep(s).describe());
    }

    let items: Vec<Either<i32, &str>> = vec![Either::Left(1), Either::Right("x")];
    println!("{items:?}");
}

fn dump<T>(value: T)
where
    T: Debug + Clone,
{
    let copy = value.clone();
    println!("dump {copy:?}");
}

fn count_by<T, K, F>(items: &[T], key: F) -> HashMap<K, usize>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut out = HashMap::new();
    for i in items {
        *out.entry(key(i)).or_insert(0) += 1;
    }

    out
}

// Example 4: where clauses with several bounds.
fn demo_where() {
    println!("-- where clauses --");

    dump(vec![1, 2, 3]);
    dump("text");

    let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
    let mut counts: Vec<_> =
        count_by(&words, |w| w.chars().next().unwrap()).into_iter().collect();
    counts.sort();
    println!("by first letter {counts:?}");
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, t: T) {
        self.items.push(t);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

// Extra methods only when T can be printed.
impl<T: Display> Stack<T> {
    fn render(&self) -> String {
        self.items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" | ")
    }
}

// Example 5: a generic container with conditional methods.
fn demo_container() {
    println!("-- generic container --");

    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("render {} peek {:?}", s.render(), s.peek());
    println!("pop {:?} now {}", s.pop(), s.render());

    let mut names: Stack<String> = Stack::new();
    names.push("ada".into());
    println!("{}", names.render());
}

// Example 6: monomorphization is visible through type_name.
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn demo_monomorphization() {
    println!("-- monomorphization --");

    println!("{} {} {}", type_of(&1u8), type_of(&"s"), type_of(&vec![Some(1.0)]));
    println!(
        "Point<u8> {} bytes, Point<f64> {} bytes",
        size_of::<Point<u8>>(),
        size_of::<Point<f64>>()
    );
}

fn main() {
    demo_functions();
    demo_structs();
    demo_enums();
    demo_where();
    demo_container();
    demo_monomorphization();
}
