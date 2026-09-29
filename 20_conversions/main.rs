// 20. Conversions: From, Into, TryFrom, and FromStr
//
// From<T> for U builds a U from a T, and can never fail. Into is the mirror image and is
// implemented for you whenever From exists, so implement From and call whichever reads
// better. .into() often needs a type annotation, because many targets are possible.
//
// TryFrom is the fallible version, with an associated Error type. Use it instead of as
// when a value that does not fit should be reported rather than truncated.
//
// FromStr parses a &str into a value; str::parse calls it. Implement it for your own
// types and parse works on them too.
//
// Run: cargo run --bin 20_conversions

use std::str::FromStr;

#[derive(Debug)]
struct Id(u32);

impl From<u32> for Id {
    fn from(n: u32) -> Self {
        Id(n)
    }
}

// Example 1: implementing From, and getting Into for free.
fn demo_from_into() {
    println!("-- From and Into --");

    let a = Id::from(7);
    let b: Id = 8.into();
    println!("{a:?} {b:?} inner {} {}", a.0, b.0);

    // std has many From impls.
    let s: String = "text".into();
    let v: Vec<u8> = "abc".into();
    let boxed: Box<str> = String::from("boxed").into();
    let wide: i64 = 5i32.into();
    let arr_vec: Vec<i32> = [1, 2, 3].into();
    println!("{s} {v:?} {boxed} {wide} {arr_vec:?}");
}

fn describe(id: impl Into<Id>) -> String {
    let id = id.into();
    format!("id #{}", id.0)
}

// Example 2: Into in a parameter makes an API accept several types.
fn demo_into_params() {
    println!("-- Into params --");

    println!("{} {}", describe(3u32), describe(Id(4)));

    fn greet(name: impl Into<String>) -> String {
        let mut s = name.into();
        s.insert_str(0, "hello ");
        s
    }

    println!("{} {}", greet("ada"), greet(String::from("bob")));
}

#[derive(Debug, PartialEq)]
struct Percent(u8);

impl TryFrom<i32> for Percent {
    type Error = String;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&n) {
            Ok(Percent(n as u8))
        } else {
            Err(format!("{n} is not 0..=100"))
        }
    }
}

// Example 3: TryFrom and TryInto.
fn demo_try_from() {
    println!("-- TryFrom --");

    let ok = u8::try_from(12i32);
    let bad = u8::try_from(300i32);
    println!("{ok:?} {bad:?}");

    let n: Result<u8, _> = 40i32.try_into();
    println!("{n:?}");

    println!("{:?} {:?}", Percent::try_from(55), Percent::try_from(140));
    let p: Result<Percent, _> = (-1).try_into();
    println!("{p:?}");

    // Slices to arrays are a common TryFrom.
    let bytes = [1u8, 2, 3, 4, 5];
    let head: [u8; 4] = bytes[..4].try_into().unwrap();
    println!("u32 from bytes {}", u32::from_be_bytes(head));

    let wrong: Result<[u8; 4], _> = bytes[..3].try_into();
    println!("wrong length {}", wrong.is_err());
}

#[derive(Debug)]
struct Rgb(u8, u8, u8);

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').ok_or("missing #")?;
        if hex.len() != 6 {
            return Err(format!("expected 6 hex digits, got {}", hex.len()));
        }

        let part =
            |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string());
        Ok(Rgb(part(0)?, part(2)?, part(4)?))
    }
}

// Example 4: FromStr for std types and your own.
fn demo_from_str() {
    println!("-- FromStr --");

    let n = i32::from_str("42").unwrap();
    let flag = bool::from_str("true").unwrap();
    let ip: std::net::Ipv4Addr = "127.0.0.1".parse().unwrap();
    println!("{n} {flag} {ip} loopback? {}", ip.is_loopback());

    for s in ["#ff8800", "ff8800", "#ff88", "#gg0000"] {
        println!("{s:<8} -> {:?}", s.parse::<Rgb>());
    }

    if let Ok(Rgb(r, g, b)) = "#102030".parse() {
        println!("channels r={r} g={g} b={b}");
    }
}

// Example 5: converting to and from strings and bytes.
fn demo_string_conversions() {
    println!("-- string conversions --");

    let n = 255;
    let s = n.to_string();
    let back: i32 = s.parse().unwrap();
    println!("{s} -> {back}");

    let bytes = "hi".as_bytes();
    let owned = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{bytes:?} -> {owned}");

    let chars: Vec<char> = "abc".chars().collect();
    let joined: String = chars.iter().collect();
    println!("{chars:?} -> {joined}");
    println!("le bytes {:?} be bytes {:?}", 1u16.to_le_bytes(), 1u16.to_be_bytes());
}

// Example 6: From between your own types, used by ?.
#[derive(Debug)]
struct Meters(f64);
#[derive(Debug)]
struct Feet(f64);

impl From<Feet> for Meters {
    fn from(f: Feet) -> Self {
        Meters(f.0 * 0.3048)
    }
}

fn demo_custom_pairs() {
    println!("-- type to type --");

    let m: Meters = Feet(10.0).into();
    println!("{m:?}");

    let heights: Vec<Meters> =
        vec![Feet(1.0), Feet(6.0)].into_iter().map(Meters::from).collect();
    println!("{heights:?}");
    println!("tallest {:.2}m", heights.iter().map(|m| m.0).fold(0.0, f64::max));
}

fn main() {
    demo_from_into();
    demo_into_params();
    demo_try_from();
    demo_from_str();
    demo_string_conversions();
    demo_custom_pairs();
}
