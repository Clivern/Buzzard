// 52. std::mem: sizes, swap, replace, take, drop, forget
//
// size_of::<T>() is the size of a type in bytes, and align_of::<T>() its alignment. Both
// are in the prelude. size_of_val looks at a value, which matters for slices and other
// unsized data. Rust may reorder struct fields to reduce padding.
//
// The three functions for moving values out of a &mut are the most useful part of the
// module. swap(a, b) exchanges two values. replace(dest, new) puts a new value in and
// hands back the old one. take(dest) is replace with Default::default(), leaving an
// empty Vec or String behind. They let you move out of a struct field you only borrow.
//
// drop(v) ends a value early. forget(v) never runs its destructor, leaking it; that is
// safe but rarely what you want. discriminant tells two enum values' variants apart
// without comparing their data.
//
// Run: cargo run --bin 52_mem

use std::mem;

// Example 1: sizes and alignment.
fn demo_sizes() {
    println!("-- sizes --");

    println!(
        "u8 {} i32 {} u64 {} u128 {} char {} bool {}",
        size_of::<u8>(),
        size_of::<i32>(),
        size_of::<u64>(),
        size_of::<u128>(),
        size_of::<char>(),
        size_of::<bool>()
    );
    println!(
        "&u8 {} &[u8] {} &str {} Box<u8> {}",
        size_of::<&u8>(),
        size_of::<&[u8]>(),
        size_of::<&str>(),
        size_of::<Box<u8>>()
    );
    println!(
        "String {} Vec<u64> {} Option<String> {}",
        size_of::<String>(),
        size_of::<Vec<u64>>(),
        size_of::<Option<String>>()
    );
    println!(
        "() {} [u32; 4] {} (u8, u32) {}",
        size_of::<()>(),
        size_of::<[u32; 4]>(),
        size_of::<(u8, u32)>()
    );
    println!(
        "align u8 {} u32 {} u64 {}",
        align_of::<u8>(),
        align_of::<u32>(),
        align_of::<u64>()
    );
}

#[allow(dead_code)]
struct Padded {
    a: u8,
    b: u64,
    c: u8,
}

// Example 2: field reordering and size_of_val.
fn demo_layout() {
    println!("-- layout --");

    // In declaration order this would be 24 bytes; Rust reorders to 16.
    println!("Padded {} bytes (fields u8, u64, u8)", size_of::<Padded>());

    let v = vec![1u32; 10];
    println!(
        "size_of_val(&v) {} (the Vec header) slice {}",
        size_of_val(&v),
        size_of_val(&v[..])
    );
    println!("size_of_val(\"hello\") {}", size_of_val("hello"));
}

// Example 3: swap and replace.
fn demo_swap_replace() {
    println!("-- swap / replace --");

    let mut a = 1;
    let mut b = 2;
    mem::swap(&mut a, &mut b);
    println!("{a} {b}");

    let mut v = vec![String::from("x"), String::from("y")];
    v.swap(0, 1); // for elements of one slice, use the slice method
    println!("{v:?}");

    let old = mem::replace(&mut a, 9);
    println!("old {old} now {a}");
}

// Example 4: take and replace for moving out of a borrowed struct.
#[derive(Debug, Default)]
struct Buffer {
    lines: Vec<String>,
    title: String,
}

impl Buffer {
    // Hand the lines to the caller and leave an empty Vec behind.
    fn flush(&mut self) -> Vec<String> {
        mem::take(&mut self.lines)
        // self.lines // error[E0507]: cannot move out of `self.lines` which is behind a mutable reference
    }

    fn rename(&mut self, new: &str) -> String {
        mem::replace(&mut self.title, new.to_string())
    }
}

fn demo_take() {
    println!("-- take --");

    let mut v = vec![1, 2, 3];
    let taken = mem::take(&mut v);
    println!("{taken:?} left {v:?}");

    let mut buf = Buffer { lines: vec!["a".into(), "b".into()], title: "draft".into() };
    let flushed = buf.flush();
    let old_title = buf.rename("final");
    println!("flushed {flushed:?} old title {old_title} now {buf:?}");

    let mut opt = Some(5);
    println!("Option::take {:?} left {opt:?}", opt.take());
}

// Example 5: a state machine that moves out of self with replace.
enum State {
    Idle,
    Running { job: String },
    Done { job: String, ok: bool },
}

impl State {
    fn advance(&mut self) {
        *self = match mem::replace(self, State::Idle) {
            State::Idle => State::Running { job: "build".into() },
            State::Running { job } => State::Done { job, ok: true },
            done @ State::Done { .. } => done,
        };
    }

    fn label(&self) -> String {
        match self {
            State::Idle => "idle".into(),
            State::Running { job } => format!("running {job}"),
            State::Done { job, ok } => format!("done {job} ok={ok}"),
        }
    }
}

fn demo_state_replace() {
    println!("-- replace in a state machine --");

    let mut s = State::Idle;
    for _ in 0..3 {
        s.advance();
        println!("{}", s.label());
    }
}

struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

// Example 6: drop, forget, and needs_drop.
fn demo_drop_forget() {
    println!("-- drop / forget --");

    let a = Loud("a");
    let b = Loud("b");
    drop(a);
    mem::forget(b); // b's destructor never runs
    println!(
        "  needs_drop String {} i32 {}",
        mem::needs_drop::<String>(),
        mem::needs_drop::<i32>()
    );
}

// Example 7: comparing enum variants with discriminant.
#[derive(Debug)]
#[allow(dead_code)] // the fields are only here to show that discriminant ignores them
enum Event {
    Click { x: i32, y: i32 },
    Key(char),
}

fn demo_discriminant() {
    println!("-- discriminant --");

    let a = Event::Click { x: 1, y: 2 };
    let b = Event::Click { x: 9, y: 9 };
    let c = Event::Key('q');
    println!(
        "{a:?} vs {b:?}: same variant? {}",
        mem::discriminant(&a) == mem::discriminant(&b)
    );
    println!(
        "{a:?} vs {c:?}: same variant? {}",
        mem::discriminant(&a) == mem::discriminant(&c)
    );
}

fn main() {
    demo_sizes();
    demo_layout();
    demo_swap_replace();
    demo_take();
    demo_state_replace();
    demo_drop_forget();
    demo_discriminant();
}
