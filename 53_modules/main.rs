// 53. Modules, use, visibility, cfg, include_str, and Cargo
//
// A module groups items under a name. mod name { ... } defines one inline. mod name;
// loads it from a file: name.rs next to the current file, or name/mod.rs. A module's
// own submodules live in a folder named after it (shapes.rs plus shapes/circle.rs in
// this lesson). Paths name items: crate:: starts at the crate root, super:: goes to the
// parent, and self:: is the current module.
//
// Everything is private by default. pub makes an item visible to the parent module and
// beyond; pub(crate) limits it to this crate and pub(super) to the parent. Struct fields
// are private unless marked pub, which is how a type protects its invariants. use brings
// a path into scope, as renames it, and pub use re-exports it under a shorter path.
//
// #[cfg(...)] compiles an item only when a condition holds (a platform, test builds, a
// Cargo feature), and cfg!(...) is the same test as a bool. include_str! and
// include_bytes! embed a file into the binary at compile time.
//
// This repository is one Cargo package with many [[bin]] targets, one per lesson.
// cargo run --bin NAME builds and runs one of them; cargo check --bin NAME only
// type-checks it, which is faster.
//
// Run: cargo run --bin 53_modules

mod greet;
mod shapes;

use std::collections::HashMap as Map;
use std::fmt::{self, Display};
use std::io::{self, Write};

// Example 1: modules from files.
fn demo_file_modules() {
    println!("-- file modules --");

    println!("{}", greet::hello("Ada"));
    let sq = shapes::Square { side: 1.5 };
    let c = shapes::Circle { r: 1.0 }; // via the pub use re-export
    let c2 = shapes::circle::Circle { r: 2.0 }; // via the full path
    println!("square {} circle {} circle2 {}", sq.area(), c.area(), c2.area());

    // shapes::helpers::round2(1.0);
    // error[E0603]: module `helpers` is private
}

mod bank {
    pub struct Account {
        pub owner: String,
        balance: i64, // private: only code in `bank` can touch it
    }

    impl Account {
        pub fn open(owner: &str) -> Self {
            Account { owner: owner.into(), balance: 0 }
        }

        pub fn deposit(&mut self, amount: i64) {
            if amount > 0 {
                self.balance += amount;
                audit::record(amount);
            }
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }
    }

    pub(crate) fn bank_name() -> &'static str {
        "Rust Bank"
    }

    mod audit {
        use std::sync::atomic::{AtomicI64, Ordering};
        static TOTAL: AtomicI64 = AtomicI64::new(0);

        pub(super) fn record(amount: i64) {
            TOTAL.fetch_add(amount, Ordering::Relaxed);
        }

        pub(in crate::bank) fn total() -> i64 {
            TOTAL.load(Ordering::Relaxed)
        }
    }

    pub fn audited_total() -> i64 {
        self::audit::total()
    }
}

// Example 2: visibility and private fields.
fn demo_visibility() {
    println!("-- visibility --");

    let mut a = bank::Account::open("ada");
    a.deposit(50);
    a.deposit(-10); // rejected by the method, which is the point of a private field
    println!("{} has {} at {}", a.owner, a.balance(), bank::bank_name());
    println!("audited {}", bank::audited_total());

    // a.balance = 1_000_000;
    // error[E0616]: field `balance` of struct `Account` is private
    // bank::Account { owner: "x".into(), balance: 5 };
    // error[E0451]: field `balance` of struct `Account` is private
}

mod geometry {
    pub mod units {
        pub const CM_PER_INCH: f64 = 2.54;
    }

    pub fn to_cm(inches: f64) -> f64 {
        inches * units::CM_PER_INCH
    }

    pub mod nested {
        pub fn from_root() -> f64 {
            crate::geometry::to_cm(1.0) + super::units::CM_PER_INCH
        }
    }
}

// Example 3: paths: crate::, super::, self::, and use.
fn demo_paths() {
    println!("-- paths and use --");

    use geometry::units::CM_PER_INCH;
    use geometry::{nested, to_cm};
    println!("{} {} {}", to_cm(10.0), CM_PER_INCH, nested::from_root());

    let mut m: Map<&str, i32> = Map::new(); // HashMap renamed with `as`
    m.insert("n", 1);
    writeln!(io::stdout(), "{m:?}").unwrap(); // io and Write from a grouped use
}

struct Version(u32, u32);

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}", self.0, self.1)
    }
}

// Example 4: cfg attributes and the cfg! macro.
#[cfg(unix)]
fn platform() -> &'static str {
    "unix"
}

#[cfg(not(unix))]
fn platform() -> &'static str {
    "not unix"
}

fn demo_cfg() {
    println!("-- cfg --");

    println!("platform {} os {}", platform(), std::env::consts::OS);
    println!("debug build? {}", cfg!(debug_assertions));
    println!("64-bit? {}", cfg!(target_pointer_width = "64"));

    if cfg!(target_os = "macos") {
        println!("hello from macOS");
    } else if cfg!(target_os = "linux") {
        println!("hello from Linux");
    }

    println!("version {}", Version(1, 2));
}

// Example 5: embedding files at compile time.
fn demo_include() {
    println!("-- include_str / include_bytes --");

    let note: &'static str = include_str!("note.txt");
    let bytes: &'static [u8] = include_bytes!("note.txt");
    println!("{:?} ({} bytes)", note.trim(), bytes.len());
}

// Example 6: compile-time information about the crate.
fn demo_crate_info() {
    println!("-- crate info --");

    println!("module_path {}", module_path!());
    println!("file {} line {}", file!(), line!());

    // env! reads a variable at compile time. Cargo sets CARGO_PKG_* for every build;
    // option_env! returns None instead of failing when the variable is missing.
    println!(
        "package {:?} version {:?}",
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION")
    );
    println!("try: cargo check --bin 53_modules (type-check only)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_ignores_negative() {
        let mut a = bank::Account::open("t");
        a.deposit(-5);
        assert_eq!(a.balance(), 0);
    }
}

fn main() {
    demo_file_modules();
    demo_visibility();
    demo_paths();
    demo_cfg();
    demo_include();
    demo_crate_info();
}
