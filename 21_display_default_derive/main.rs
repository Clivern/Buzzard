// 21. Display, Default, and derive
//
// Display is the user-facing format used by {}. Implement it with write!(f, "...").
// Every Display type gets to_string() for free through ToString, so never implement
// ToString yourself. Display has no derive: you decide how a value reads to a user.
//
// Default::default() builds a reasonable empty value. #[derive(Default)] works when
// every field has a Default. Combined with struct update syntax it gives you
// "defaults, except these fields".
//
// #[derive(...)] asks the compiler to write impls for you. The built-in derives are
// Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, and Hash. A derive fails
// if a field does not implement the trait.
//
// Run: cargo run --bin 21_display_default_derive

use std::fmt;

struct Meter(i32);

impl fmt::Display for Meter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

// Example 1: a minimal Display.
fn demo_display() {
    println!("-- Display --");

    let m = Meter(12);
    println!("{m}");

    let s: String = m.to_string();
    println!("to_string {s} len {}", s.len());

    // println!("{:?}", m); // error[E0277]: `Meter` doesn't implement `Debug`
}

struct Money {
    cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.abs();
        let text = format!("{sign}${}.{:02}", abs / 100, abs % 100);

        // pad respects width and alignment from the caller's format spec.
        f.pad(&text)
    }
}

// Example 2: Display that honors the caller's width and alignment.
fn demo_display_padding() {
    println!("-- Display with padding --");

    let items = [("coffee", Money { cents: 450 }), ("refund", Money { cents: -1299 })];
    for (name, price) in &items {
        println!("[{name:<8}] [{price:>10}]");
    }
}

#[derive(Debug)]
struct Matrix([[i32; 2]; 2]);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            writeln!(f, "| {:>3} {:>3} |", row[0], row[1])?;
        }

        Ok(())
    }
}

// Example 3: multi-line Display, and Debug next to it.
fn demo_display_vs_debug() {
    println!("-- Display vs Debug --");

    let m = Matrix([[1, 20], [300, 4]]);
    print!("{m}");
    println!("{m:?}");

    // The alternate flag {:#} is available inside fmt as f.alternate().
}

#[derive(Debug, Default)]
struct Cfg {
    host: String,
    port: u16,
    verbose: bool,
    tags: Vec<String>,
    timeout: Option<u32>,
}

// Example 4: derived Default.
fn demo_default() {
    println!("-- Default --");

    let d = Cfg::default();
    println!("{d:?}");

    let c = Cfg { port: 8080, ..Default::default() };
    println!("{c:?}");
    println!(
        "host {:?} port {} verbose {} tags {} timeout {:?}",
        c.host,
        c.port,
        c.verbose,
        c.tags.len(),
        c.timeout
    );
    println!(
        "std defaults: {:?} {:?} {:?} {:?}",
        i32::default(),
        String::default(),
        <Option<u8>>::default(),
        <(bool, char)>::default()
    );
}

#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    workers: usize,
}

impl Default for Server {
    fn default() -> Self {
        Server { host: "0.0.0.0".into(), port: 80, workers: 4 }
    }
}

#[derive(Debug, Default, PartialEq)]
enum Mode {
    #[default]
    Fast,
    Safe,
}

// Example 5: hand-written Default, and #[default] on an enum variant.
fn demo_custom_default() {
    println!("-- custom Default --");

    let s = Server { port: 8443, ..Default::default() };
    println!("{s:?}");
    println!("{} {} {}", s.host, s.port, s.workers);
    println!("mode {:?} safe? {}", Mode::default(), Mode::default() == Mode::Safe);

    let counts: std::collections::HashMap<&str, i32> = Default::default();
    println!("empty map {counts:?}");
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Version {
    major: u32,
    minor: u32,
}

// Example 6: what derive generates.
fn demo_derive() {
    println!("-- derive --");

    let a = Version { major: 1, minor: 2 };
    let b = a.clone();
    println!("{:?} == {:?}? {}", a, b, a == b);

    let newer = Version { major: 1, minor: 10 };

    // Derived ordering compares fields top to bottom.
    println!("a < newer? {} max {:?}", a < newer, std::cmp::max(&a, &newer));

    let mut set = std::collections::HashSet::new();
    set.insert(a);
    set.insert(b);
    println!("set len {} default {:?}", set.len(), Version::default());

    // #[derive(Clone)] struct NoClone { f: std::sync::MutexGuard<'static, ()> }
    // error[E0277]: the trait bound `std::sync::MutexGuard<'_, ()>: Clone` is not satisfied
}

fn main() {
    demo_display();
    demo_display_padding();
    demo_display_vs_debug();
    demo_default();
    demo_custom_default();
    demo_derive();
}
