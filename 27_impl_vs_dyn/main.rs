// 27. impl Trait vs dyn Trait
//
// impl Trait in argument position is shorthand for a generic: the caller picks one
// concrete type, and the compiler builds a specialized copy (static dispatch). In return
// position it hides the concrete type, which is how you return a closure or a long
// iterator chain without naming it. A function returning impl Trait must return one
// single concrete type.
//
// dyn Trait is a type that can hold any implementor, chosen at runtime. It always sits
// behind a pointer (&dyn, Box<dyn>, Rc<dyn>), and calls go through a vtable (dynamic
// dispatch). Use it when you need mixed types in one collection, or to pick an
// implementation at runtime.
//
// Only dyn-compatible ("object-safe") traits can be used as dyn: no generic methods,
// and no methods that return Self by value, unless those methods opt out with
// where Self: Sized.
//
// Run: cargo run --bin 27_impl_vs_dyn

use std::fmt::Display;

trait Speak {
    fn speak(&self) -> String;
    fn twice(&self) -> String {
        format!("{0} {0}", self.speak())
    }
}

struct Dog;
struct Robot {
    id: u32,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        "woof".into()
    }
}

impl Speak for Robot {
    fn speak(&self) -> String {
        format!("beep #{}", self.id)
    }
}

// Example 1: impl Trait arguments are generics in disguise.
fn say_static(s: &impl Speak) -> String {
    s.speak()
}

fn say_generic<S: Speak>(s: &S) -> String {
    s.speak()
}

fn add_one(x: impl Into<i32>) -> i32 {
    x.into() + 1
}

fn demo_impl_args() {
    println!("-- impl Trait args --");

    println!("{} {}", say_static(&Dog), say_generic(&Robot { id: 1 }));
    println!("{} {}", add_one(3u8), add_one(-2i16));
}

// Example 2: impl Trait in return position hides the type.
fn odds(limit: i32) -> impl Iterator<Item = i32> {
    (0..limit).filter(|n| n % 2 == 1)
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn label(n: i32) -> impl Display {
    format!("<{n}>")
}

fn demo_impl_return() {
    println!("-- impl Trait returns --");

    println!("{:?}", odds(10).collect::<Vec<_>>());
    let add5 = make_adder(5);
    println!("add5(2) = {}", add5(2));
    println!("{}", label(3));

    // fn pick(b: bool) -> impl Speak { if b { Dog } else { Robot { id: 0 } } }
    // error[E0308]: `if` and `else` have incompatible types
}

// Example 3: &dyn Trait and Box<dyn Trait>.
fn say_dyn(s: &dyn Speak) -> String {
    s.twice()
}

fn pick(loud: bool) -> Box<dyn Speak> {
    if loud { Box::new(Robot { id: 9 }) } else { Box::new(Dog) }
}

fn demo_dyn() {
    println!("-- dyn Trait --");

    println!("{} / {}", say_dyn(&Dog), say_dyn(&Robot { id: 2 }));
    println!("{} / {}", pick(true).speak(), pick(false).speak());
}

// Example 4: heterogeneous collections.
fn demo_collections() {
    println!("-- mixed collection --");

    let list: Vec<Box<dyn Speak>> =
        vec![Box::new(Dog), Box::new(Robot { id: 3 }), Box::new(Dog)];
    for s in &list {
        println!("{}", s.speak());
    }

    let printable: Vec<&dyn Display> = vec![&1, &"two", &3.5];
    let joined: Vec<String> = printable.iter().map(|d| d.to_string()).collect();
    println!("{}", joined.join(", "));

    // Picking behaviour at runtime: a map of strategies.
    let ops: Vec<(&str, Box<dyn Fn(i32, i32) -> i32>)> = vec![
        ("add", Box::new(|a, b| a + b)),
        ("mul", Box::new(|a, b| a * b)),
        ("max", Box::new(std::cmp::max)),
    ];

    for (name, f) in &ops {
        println!("{name}(6, 7) = {}", f(6, 7));
    }
}

// Example 5: the cost: fat pointers and vtables.
fn demo_sizes() {
    println!("-- sizes --");

    println!(
        "&Dog {} bytes, &dyn Speak {} bytes",
        size_of::<&Dog>(),
        size_of::<&dyn Speak>()
    );
    println!(
        "Box<Robot> {} bytes, Box<dyn Speak> {} bytes",
        size_of::<Box<Robot>>(),
        size_of::<Box<dyn Speak>>()
    );

    // The second word is the vtable pointer: method addresses plus size and alignment.
}

// Example 6: dyn compatibility.
trait Shape {
    fn area(&self) -> f64;

    // Returning Self would make the trait unusable as dyn, unless Self: Sized opts out.
    fn unit() -> Self
    where
        Self: Sized;
    fn scaled_area(&self, k: f64) -> f64 {
        self.area() * k * k
    }
}

struct Sq(f64);

impl Shape for Sq {
    fn area(&self) -> f64 {
        self.0 * self.0
    }

    fn unit() -> Self {
        Sq(1.0)
    }
}

// trait Cloner { fn dup(&self) -> Self; }
// fn take(_: &dyn Cloner) {}
// error[E0038]: the trait `Cloner` is not dyn compatible

fn demo_dyn_compatibility() {
    println!("-- dyn compatibility --");

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Sq(2.0)), Box::new(Sq::unit())];
    for s in &shapes {
        println!("area {} scaled {}", s.area(), s.scaled_area(3.0));
    }
}

// Example 7: when to pick which.
fn demo_choosing() {
    println!("-- choosing --");

    // Static: one type per call site, inlined, no allocation.
    let total: usize = [Dog, Dog].iter().map(say_static).map(|s| s.len()).sum();

    // Dynamic: one compiled function, many types at runtime.
    let mixed: usize =
        [&Dog as &dyn Speak, &Robot { id: 4 }].iter().map(|s| say_dyn(*s).len()).sum();
    println!("static total {total} dynamic total {mixed}");
}

fn main() {
    demo_impl_args();
    demo_impl_return();
    demo_dyn();
    demo_collections();
    demo_sizes();
    demo_dyn_compatibility();
    demo_choosing();
}
