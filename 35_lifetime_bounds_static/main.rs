// 35. 'static and lifetime bounds
//
// &'static T is a reference that is valid for the rest of the program. String literals
// are &'static str because they are baked into the binary. A static item also lives
// forever.
//
// The bound T: 'static is different and more common. It does not mean "lives forever";
// it means T holds no borrows shorter than 'static. Every owned type with no references
// inside (String, Vec<i32>, Box<u8>) is 'static. thread::spawn requires F: 'static
// because the new thread might outlive the caller's stack.
//
// T: 'a means every reference inside T is valid for at least 'a. 'a: 'b means 'a
// outlives 'b. Trait objects carry a lifetime too: Box<dyn Trait> means Box<dyn Trait +
// 'static>, and Box<dyn Trait + 'a> allows borrows.
//
// Run: cargo run --bin 35_lifetime_bounds_static

use std::fmt::Display;

fn describe() -> &'static str {
    "ok"
}

static GREETING: &str = "hello";

// Example 1: &'static references.
fn demo_static_refs() {
    println!("-- &'static --");

    let s: &'static str = "literal";
    println!("{s} {} {GREETING}", describe());

    // Leaking a value gives a &'static on purpose (it is never freed).
    let leaked: &'static mut Vec<i32> = Box::leak(Box::new(vec![1, 2]));
    leaked.push(3);
    println!("leaked {leaked:?}");
}

fn needs_static<T: 'static + Display>(t: T) -> String {
    format!("[{t}]")
}

// Example 2: T: 'static means "owns its data", not "lives forever".
fn demo_static_bound() {
    println!("-- T: 'static --");

    let owned = String::from("dropped at end of this function");
    println!("{}", needs_static(owned)); // a String is 'static: it has no borrows
    println!("{}", needs_static(42));
    println!("{}", needs_static("literals are 'static too"));

    let local = String::from("local");
    let _r = &local;

    // needs_static(_r);
    // error[E0597]: `local` does not live long enough (a &String to a local is not 'static)
}

// Example 3: why thread::spawn needs 'static.
fn demo_threads() {
    println!("-- spawn needs 'static --");

    let data = vec![1, 2, 3];

    // std::thread::spawn(|| println!("{:?}", data));
    // error[E0373]: closure may outlive the current function, but it borrows `data`
    let h = std::thread::spawn(move || data.iter().sum::<i32>());
    println!("sum from thread {}", h.join().unwrap());

    // Scoped threads (lesson 43) lift the 'static requirement.
}

fn print_ref<'a, T: Display + 'a>(value: &'a T) -> String {
    format!("<{value}>")
}

struct Wrapper<'a, T: 'a> {
    inner: &'a T,
}

impl<'a, T: Display + 'a> Wrapper<'a, T> {
    fn show(&self) -> String {
        format!("wrapped {}", self.inner)
    }
}

// Example 4: T: 'a bounds.
fn demo_t_outlives_a() {
    println!("-- T: 'a --");

    let s = String::from("hi");
    println!("{}", print_ref(&s));

    let n = 7;
    let w = Wrapper { inner: &n };
    println!("{}", w.show());
}

// 'long: 'short lets us return the longer-lived borrow as the shorter one.
fn shorten<'long: 'short, 'short>(x: &'long str, _y: &'short str) -> &'short str {
    x
}

// Example 5: 'a: 'b outlives bounds.
fn demo_outlives() {
    println!("-- 'a: 'b --");

    let long = String::from("long lived");
    {
        let short = String::from("short");
        println!("{}", shorten(&long, &short));
    }
}

trait Render {
    fn render(&self) -> String;
}

struct Borrowed<'a>(&'a str);

impl Render for Borrowed<'_> {
    fn render(&self) -> String {
        format!("borrowed({})", self.0)
    }
}

struct Owned(String);

impl Render for Owned {
    fn render(&self) -> String {
        format!("owned({})", self.0)
    }
}

fn boxed_static() -> Box<dyn Render> {
    Box::new(Owned("x".into()))
}

fn boxed_borrow<'a>(s: &'a str) -> Box<dyn Render + 'a> {
    Box::new(Borrowed(s))
}

// Example 6: lifetimes on trait objects.
fn demo_trait_objects() {
    println!("-- dyn Trait + 'a --");

    let text = String::from("local text");
    let items: Vec<Box<dyn Render + '_>> = vec![boxed_static(), boxed_borrow(&text)];
    for i in &items {
        println!("{}", i.render());
    }

    // fn bad(s: &str) -> Box<dyn Render> { Box::new(Borrowed(s)) }
    // error: lifetime may not live long enough (Box<dyn Render> means + 'static)
}

fn main() {
    demo_static_refs();
    demo_static_bound();
    demo_threads();
    demo_t_outlives_a();
    demo_outlives();
    demo_trait_objects();
}
