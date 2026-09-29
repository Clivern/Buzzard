// 23. Drop, Deref, and ManuallyDrop
//
// Drop::drop runs when a value goes out of scope. String, Vec, File, and MutexGuard use
// it to free memory, close handles, and unlock. Locals drop in reverse declaration
// order; a struct's fields drop after its own drop, in declaration order. std::mem::drop
// (just drop) ends a value early by moving it into a function that does nothing.
//
// Deref lets a type act like a reference to something inside it. String derefs to str,
// Vec<T> to [T], Box<T> to T. Method calls follow Deref automatically, and &String
// coerces to &str where a &str is expected (deref coercion). DerefMut is the mutable
// side.
//
// ManuallyDrop<T> switches Drop off for one value. It is a tool for unsafe code and
// custom containers, not for everyday use.
//
// Run: cargo run --bin 23_drop_deref

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

// Example 1: scope ends, early drop, and order.
fn demo_drop_order() {
    println!("-- drop order --");

    let a = Guard("a");
    {
        let _b = Guard("b");
        println!("  inner scope ends");
    }

    let _c = Guard("c");
    drop(a);
    println!("  after drop(a)");

    // a.0; // error[E0382]: use of moved value: `a`
    // a.drop(); // error[E0040]: explicit use of destructor method
}

struct Outer {
    _first: Guard,
    _second: Guard,
}

impl Drop for Outer {
    fn drop(&mut self) {
        println!("  drop Outer (fields drop after this)");
    }
}

// Example 2: struct fields, Vec elements, and temporaries.
fn demo_field_order() {
    println!("-- fields and elements --");

    {
        let _o = Outer { _first: Guard("field 1"), _second: Guard("field 2") };
    }

    {
        let _v = vec![Guard("vec[0]"), Guard("vec[1]")];
        println!("  vec going out of scope");
    }

    // let _ = binds nothing, so the value drops immediately.
    let _ = Guard("temporary");
    let _kept = Guard("kept until end");
    println!("  end of demo");
}

// Example 3: a drop guard that runs cleanup on every exit path.
struct Timer {
    label: &'static str,
    start: std::time::Instant,
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("  {} cleanup ran after {:?}", self.label, self.start.elapsed());
    }
}

fn work(fail: bool) -> Result<(), &'static str> {
    let _t = Timer {
        label: if fail { "failing work" } else { "work" },
        start: std::time::Instant::now(),
    };

    if fail {
        return Err("early return still runs Drop");
    }

    Ok(())
}

fn demo_drop_guard() {
    println!("-- drop guard --");

    println!("  {:?}", work(false));
    println!("  {:?}", work(true));
}

fn needs_str(s: &str) -> usize {
    s.len()
}

fn sum(s: &[i32]) -> i32 {
    s.iter().sum()
}

// Example 4: Deref in std, and deref coercion.
fn demo_std_deref() {
    println!("-- std Deref --");

    let s = String::from("hello");
    println!("needs_str(&String) {}", needs_str(&s));

    let r: &str = s.deref();
    let also: &str = &s;
    println!("{r} {also}");

    let v = vec![1, 2, 3];
    println!("sum(&Vec) {}", sum(&v));

    let b = Box::new(5);
    println!("*b + 1 = {}", *b + 1);

    // Coercion chains: &Box<String> -> &String -> &str.
    let bs = Box::new(String::from("boxed"));
    println!("chained {}", needs_str(&bs));
}

struct Stack {
    items: Vec<i32>,
    pushes: usize,
}

impl Deref for Stack {
    type Target = Vec<i32>;
    fn deref(&self) -> &Vec<i32> {
        &self.items
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Vec<i32> {
        self.pushes += 1;
        &mut self.items
    }
}

// Example 5: implementing Deref and DerefMut.
fn demo_custom_deref() {
    println!("-- custom Deref --");

    let mut st = Stack { items: vec![], pushes: 0 };
    st.push(1); // DerefMut, then Vec::push
    st.push(2);
    println!("len {} last {:?} contains 1? {}", st.len(), st.last(), st.contains(&1));
    println!("mutable derefs {}", st.pushes);

    // Use Deref for smart-pointer-like wrappers. For a newtype that should hide its
    // inner type, prefer explicit methods instead.
}

// Example 6: ManuallyDrop.
fn demo_manually_drop() {
    println!("-- ManuallyDrop --");

    let quiet = ManuallyDrop::new(Guard("never dropped automatically"));
    println!("  holds {}", quiet.0);

    let inner: Guard = ManuallyDrop::into_inner(ManuallyDrop::new(Guard("taken back")));
    println!("  got {}", inner.0);

    // `quiet` is leaked at the end of scope; `inner` is dropped normally.
    // std::mem::forget(x) does the same as never dropping x.
    let s = ManuallyDrop::new(String::from("keep"));
    println!("  derefs to inner: {}", s.len());

    let _ = ManuallyDrop::into_inner(s);
}

fn main() {
    demo_drop_order();
    demo_field_order();
    demo_drop_guard();
    demo_std_deref();
    demo_custom_deref();
    demo_manually_drop();
}
