// 18. Result and the ? operator
//
// Result<T, E> is Ok(T) or Err(E). A function that can fail returns a Result, and the
// caller must deal with the error before reaching the value. Ok(()) means success with
// nothing to return.
//
// ? on a Result returns the Err early from the current function, converting it with
// From on the way. The current function must return a compatible Result (or Option, for
// ? on an Option). main can return Result too, so ? works at the top level.
//
// map transforms Ok, map_err transforms Err, and and_then chains another fallible step.
// unwrap and expect panic on Err: use them when an error would be a bug, not an expected
// failure.
//
// Run: cargo run --bin 18_result

use std::num::ParseIntError;

fn parse_port(s: &str) -> Result<u16, ParseIntError> {
    s.parse()
}

// Example 1: producing and matching Results.
fn demo_basics() {
    println!("-- basics --");

    println!("{:?} {:?}", parse_port("8080"), parse_port("no"));
    for input in ["443", "70000", "abc"] {
        match parse_port(input) {
            Ok(p) => println!("{input}: port {p}"),
            Err(e) => println!("{input}: error {e}"),
        }
    }

    println!("is_ok {} is_err {}", parse_port("1").is_ok(), parse_port("x").is_err());
    println!("is_ok_and {}", parse_port("22").is_ok_and(|p| p < 1024));
}

// Example 2: map, map_err, and_then, and fallbacks.
fn demo_combinators() {
    println!("-- combinators --");

    let r: Result<i32, &str> = Ok(3);
    let e: Result<i32, &str> = Err("boom");
    println!("map {:?} {:?}", r.map(|n| n + 1), e.map(|n| n + 1));
    println!("map_err {:?}", e.map_err(|e| format!("bad:{e}")));
    println!(
        "and_then {:?}",
        r.and_then(|n| if n > 5 { Ok(n) } else { Err("too small") })
    );
    println!(
        "unwrap_or {} unwrap_or_else {}",
        e.unwrap_or(0),
        e.unwrap_or_else(|m| m.len() as i32)
    );
    println!("ok {:?} err {:?}", r.ok(), e.err());
    println!("or_else {:?}", e.or_else(|_| Ok::<i32, &str>(99)));
}

fn add_texts(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

// The same function without ?, to show what it expands to.
fn add_texts_by_hand(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = match a.parse() {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };

    let y: i32 = match b.parse() {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    };

    Ok(x + y)
}

// Example 3: the ? operator.
fn demo_question() {
    println!("-- ? --");

    println!("{:?} {:?}", add_texts("2", "3"), add_texts("2", "x"));
    println!("by hand {:?}", add_texts_by_hand("10", "5"));

    // fn no_result() { let n: i32 = "1".parse()?; }
    // error[E0277]: the `?` operator can only be used in a function that returns
    // `Result` or `Option`
}

// Example 4: unwrap, expect, and the panic you get.
fn demo_unwrap() {
    println!("-- unwrap / expect --");

    let n: i32 = "12".parse().expect("digits");
    let m = Some(4).unwrap();
    println!("{n} {m}");

    // The panic message below goes to stderr; catch_unwind stops it from ending main.
    let caught = std::panic::catch_unwind(|| {
        let _: i32 = "x".parse().expect("config value must be a number");
    });

    println!("expect on Err panicked: {}", caught.is_err());
}

// Example 5: collecting many Results.
fn demo_collect() {
    println!("-- collect --");

    let good: Result<Vec<i32>, _> =
        ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let bad: Result<Vec<i32>, _> =
        ["1", "x", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{good:?} {bad:?}");

    // Keep the good values and count the bad ones.
    let (oks, errs): (Vec<_>, Vec<_>) =
        ["1", "x", "3", "y"].iter().map(|s| s.parse::<i32>()).partition(|r| r.is_ok());
    let oks: Vec<i32> = oks.into_iter().map(Result::unwrap).collect();
    println!("oks {oks:?} errors {}", errs.len());

    let sum: Result<i32, _> = ["4", "5"].iter().map(|s| s.parse::<i32>()).sum();
    println!("sum {sum:?}");
}

// Example 6: Result<(), E> for actions that can fail.
fn check_age(age: i32) -> Result<(), String> {
    if !(0..=150).contains(&age) {
        return Err(format!("age {age} out of range"));
    }

    Ok(())
}

fn demo_unit_result() {
    println!("-- Result<(), E> --");

    for age in [30, -1] {
        match check_age(age) {
            Ok(()) => println!("{age} ok"),
            Err(e) => println!("{e}"),
        }
    }

    // A Result must be used; ignoring it warns. Use let _ = to discard on purpose.
    let _ = check_age(200);
}

// Example 7: main can return a Result.
fn main() -> Result<(), ParseIntError> {
    demo_basics();
    demo_combinators();
    demo_question();
    demo_unwrap();
    demo_collect();
    demo_unit_result();

    let n: i32 = "7".parse()?;
    println!("-- main returns Result --\nparsed {n} at the top level");
    Ok(())
}
