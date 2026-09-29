// 28. Newtypes and extension traits
//
// struct Watts(u32) is a new type, not an alias. You cannot pass Watts where a u32 or a
// Volts is expected, so units and meanings stay straight. The wrapper costs nothing at
// runtime. A newtype can also enforce an invariant (a validated email, a non-empty
// list) by keeping its field private and checking in the constructor.
//
// The orphan rule says you may implement a trait for a type only if you own the trait
// or the type. A newtype gets around it: wrap the foreign type, then implement the
// foreign trait on your wrapper.
//
// An extension trait goes the other way: define your own trait and implement it for a
// type you do not own, to add methods to it. The methods are visible wherever the trait
// is in scope.
//
// Run: cargo run --bin 28_newtype_extension

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Watts(u32);
#[derive(Debug, Clone, Copy)]
struct Volts(u32);

impl fmt::Display for Watts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}W", self.0)
    }
}

fn draw(power: Watts) -> String {
    format!("drawing {power}")
}

// Example 1: newtypes keep units apart.
fn demo_units() {
    println!("-- units --");

    println!("{}", draw(Watts(60)));
    let v = Volts(230);

    // draw(v); // error[E0308]: expected `Watts`, found `Volts`
    // draw(60); // error[E0308]: expected `Watts`, found integer
    println!("volts {}", v.0);
    println!("zero cost: {} bytes", size_of::<Watts>());
}

mod email {
    #[derive(Debug, Clone)]
    pub struct Email(String);

    impl Email {
        pub fn parse(s: &str) -> Result<Self, String> {
            match s.split_once('@') {
                Some((user, domain)) if !user.is_empty() && domain.contains('.') => {
                    Ok(Email(s.to_lowercase()))
                }
                _ => Err(format!("{s:?} is not an email")),
            }
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }

        pub fn domain(&self) -> &str {
            self.0.split_once('@').map(|(_, d)| d).unwrap_or("")
        }
    }
}

// Example 2: a newtype that guarantees an invariant.
fn demo_validated() {
    println!("-- validated newtype --");

    for s in ["Ada@Example.com", "nope", "@x.io"] {
        match email::Email::parse(s) {
            Ok(e) => println!("ok {} domain {}", e.as_str(), e.domain()),
            Err(msg) => println!("{msg}"),
        }
    }

    // email::Email("bad".into()); // error[E0603]: tuple struct constructor `Email` is private
}

// impl fmt::Display for Vec<String> {}
// error[E0117]: only traits defined in the current crate can be implemented for types
// defined outside of the crate

struct List(Vec<String>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Example 3: the newtype gets around the orphan rule.
fn demo_orphan() {
    println!("-- orphan rule --");

    let l = List(vec!["a".into(), "b".into()]);
    println!("{l} has {} items", l.0.len());
}

// Example 4: exposing the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UserId(u64);

impl From<u64> for UserId {
    fn from(n: u64) -> Self {
        UserId(n)
    }
}

impl UserId {
    fn get(self) -> u64 {
        self.0
    }
}

fn demo_inner_access() {
    println!("-- inner access --");

    let id: UserId = 42.into();
    let UserId(raw) = id;
    println!("{id:?} get {} destructured {raw}", id.get());

    let mut ids = vec![UserId(3), UserId(1), UserId(2)];
    ids.sort();
    println!("derived Ord works: {ids:?}");
}

trait Hex {
    fn hex(&self) -> String;
}

impl Hex for u8 {
    fn hex(&self) -> String {
        format!("{self:02x}")
    }
}

impl Hex for [u8] {
    fn hex(&self) -> String {
        self.iter().map(|b| b.hex()).collect()
    }
}

// Example 5: extension traits add methods to foreign types.
fn demo_extension() {
    println!("-- extension traits --");

    println!("{} {}", 15u8.hex(), b"Rust"[..].hex());
}

trait StrExt {
    fn word_count(&self) -> usize;
    fn title_case(&self) -> String;
}

impl StrExt for str {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }

    fn title_case(&self) -> String {
        self.split_whitespace()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

trait IterExt: Iterator {
    fn sum_squares(self) -> i64
    where
        Self: Sized,
        Self::Item: Into<i64>,
    {
        self.map(|x| {
            let x: i64 = x.into();
            x * x
        })
        .sum()
    }
}

// A blanket impl: every iterator gets the method.
impl<I: Iterator> IterExt for I {}

// Example 6: blanket extension traits.
fn demo_blanket_extension() {
    println!("-- blanket extension --");

    let s = "the quick brown fox";
    println!("{} words, {}", s.word_count(), s.title_case());
    println!("String derefs to str: {}", String::from("a b").word_count());
    println!("sum of squares {}", [1, 2, 3].into_iter().sum_squares());
    println!("over a range {}", (1u8..=4).sum_squares());
}

fn main() {
    demo_units();
    demo_validated();
    demo_orphan();
    demo_inner_access();
    demo_extension();
    demo_blanket_extension();
}
