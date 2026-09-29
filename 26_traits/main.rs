// 26. Traits
//
// A trait is a set of methods a type can implement: impl Trait for Type. There is no
// implements keyword on the type itself. A trait can give methods default bodies, and
// those defaults can call the methods implementors must write; that is how
// Iterator::map and friends are built on top of next.
//
// You can call a trait's methods only when the trait is in scope (use std::io::Write
// before writeln! on a File, for example). A supertrait, trait Label: Display, means
// only Display types may implement Label. An associated type, type Item;, lets each
// impl choose one type that the trait's methods talk about. Traits can also have
// associated functions and constants.
//
// Run: cargo run --bin 26_traits

use std::fmt::Display;

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn total_area<T: Area>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Example 1: defining and implementing a trait.
fn demo_basics() {
    println!("-- basics --");

    println!(
        "circle {:.2} square {}",
        Circle { r: 2.0 }.area(),
        Square { side: 3.0 }.area()
    );
    println!("total {:.2}", total_area(&[Circle { r: 1.0 }, Circle { r: 2.0 }]));
}

trait Greet {
    fn name(&self) -> String;

    fn hello(&self) -> String {
        format!("hello {}", self.name())
    }

    fn shout(&self) -> String {
        self.hello().to_uppercase()
    }
}

struct User(&'static str);
struct Bot;

impl Greet for User {
    fn name(&self) -> String {
        self.0.to_string()
    }
}

impl Greet for Bot {
    fn name(&self) -> String {
        "bot".into()
    }

    // Override one default and keep the other.
    fn hello(&self) -> String {
        "beep boop".into()
    }
}

// Example 2: default methods, overriding, and defaults calling other methods.
fn demo_defaults() {
    println!("-- default methods --");

    println!("{} / {}", User("Ada").hello(), User("Ada").shout());
    println!("{} / {}", Bot.hello(), Bot.shout());
}

trait Label: Display {
    fn label(&self) -> String {
        format!("item:{self}")
    }
}

impl Label for i32 {}
impl Label for &str {}

// Example 3: supertraits, and implementing your trait for std types.
fn demo_supertraits() {
    println!("-- supertraits --");

    println!("{} {}", 7.label(), "x".label());

    // struct NoDisplay; impl Label for NoDisplay {}
    // error[E0277]: `NoDisplay` doesn't implement `std::fmt::Display`
}

trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }
}

struct Shelf {
    books: Vec<String>,
}

impl Container for Shelf {
    type Item = String;
    fn get(&self, i: usize) -> Option<&String> {
        self.books.get(i)
    }
}

impl<T> Container for Vec<T> {
    type Item = T;
    fn get(&self, i: usize) -> Option<&T> {
        self.as_slice().get(i)
    }
}

fn show_first<C>(c: &C) -> String
where
    C: Container,
    C::Item: std::fmt::Debug,
{
    format!("{:?}", c.first())
}

// Example 4: associated types.
fn demo_associated_types() {
    println!("-- associated types --");

    let shelf = Shelf { books: vec!["Dune".into(), "Emma".into()] };
    println!("{} {}", show_first(&shelf), show_first(&vec![10, 20]));
    println!("{:?}", Container::get(&shelf, 1));
}

trait Shape {
    const SIDES: u32;
    fn new_unit() -> Self;
    fn describe() -> String {
        format!("{} sides", Self::SIDES)
    }
}

struct Triangle;

impl Shape for Triangle {
    const SIDES: u32 = 3;
    fn new_unit() -> Self {
        Triangle
    }
}

impl Shape for Square {
    const SIDES: u32 = 4;
    fn new_unit() -> Self {
        Square { side: 1.0 }
    }
}

fn make<S: Shape>() -> S {
    S::new_unit()
}

// Example 5: associated consts and functions.
fn demo_associated_items() {
    println!("-- associated consts and fns --");

    println!("triangle {} square {}", Triangle::describe(), Square::describe());
    let sq: Square = make();
    let _t: Triangle = make();
    println!("unit square area {}", sq.area());
}

mod shapes {
    pub trait Perimeter {
        fn perimeter(&self) -> f64;
    }

    impl Perimeter for super::Square {
        fn perimeter(&self) -> f64 {
            self.side * 4.0
        }
    }
}

// Example 6: a trait must be in scope to call its methods.
fn demo_scope() {
    println!("-- trait in scope --");

    let sq = Square { side: 2.5 };

    // sq.perimeter(); // without the use: error[E0599]: no method named `perimeter`
    use shapes::Perimeter;
    println!("perimeter {}", sq.perimeter());

    // Fully qualified syntax picks a trait explicitly.
    println!("qualified {}", <Square as shapes::Perimeter>::perimeter(&sq));
}

trait Pilot {
    fn fly(&self) -> &'static str;
}
trait Wizard {
    fn fly(&self) -> &'static str;
}
struct Human;
impl Pilot for Human {
    fn fly(&self) -> &'static str {
        "this is your captain speaking"
    }
}
impl Wizard for Human {
    fn fly(&self) -> &'static str {
        "up!"
    }
}

// Example 7: two traits with the same method name.
fn demo_disambiguation() {
    println!("-- disambiguation --");

    let h = Human;
    println!("{} / {}", Pilot::fly(&h), Wizard::fly(&h));
}

fn main() {
    demo_basics();
    demo_defaults();
    demo_supertraits();
    demo_associated_types();
    demo_associated_items();
    demo_scope();
    demo_disambiguation();
}
