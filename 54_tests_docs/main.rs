//! A tiny documented binary. `//!` documents the enclosing item: here, the crate.

// 54. Tests, doc comments, assertions, and todo!
//
// #[test] marks a function for cargo test. A test passes if it returns normally and
// fails if it panics, so assert!, assert_eq!, and assert_ne! are the usual checks; they
// print both sides on failure. #[should_panic] expects a panic, a test can return
// Result and use ?, and #[ignore] skips a slow test unless asked. Unit tests live in a
// #[cfg(test)] mod tests next to the code; tests/ at the crate root holds integration
// tests.
//
// /// documents the next item and //! the enclosing one; cargo doc renders them. Code
// blocks in doc comments are doctests, which cargo test runs for library crates (not
// binaries like this one).
//
// assert! states an invariant; debug_assert! is the same check in debug builds only.
// todo! marks unfinished code, unimplemented! a deliberately missing feature, and
// unreachable! a branch that cannot happen. All of them panic if reached.
//
// Run: cargo run --bin 54_tests_docs
// Test: cargo test --bin 54_tests_docs

use std::collections::HashMap;

/// Sums the numbers in `nums`.
///
/// # Examples
///
/// ```
/// assert_eq!(2 + 2, 4);
/// ```
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

/// Splits `"key=value"` into its parts.
///
/// Returns `None` when there is no `=`. See also [`parse_all`].
fn parse_pair(s: &str) -> Option<(&str, &str)> {
    let (k, v) = s.split_once('=')?;
    Some((k.trim(), v.trim()))
}

/// Parses every line with [`parse_pair`], skipping blank lines.
///
/// # Errors
///
/// Returns the first line that is not a valid pair.
fn parse_all(text: &str) -> Result<HashMap<&str, &str>, String> {
    let mut out = HashMap::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let (k, v) = parse_pair(line).ok_or_else(|| format!("bad line: {line:?}"))?;
        out.insert(k, v);
    }

    Ok(out)
}

/// Divides, panicking on zero.
///
/// # Panics
///
/// Panics if `b` is zero.
fn divide(a: i32, b: i32) -> i32 {
    assert!(b != 0, "division by zero: {a} / {b}");
    a / b
}

// Example 1: the code under test.
fn demo_code() {
    println!("-- code under test --");

    println!("sum {}", sum(&[1, 2, 3]));
    println!("{:?} {:?}", parse_pair("a = 1"), parse_pair("nope"));

    let cfg = parse_all("host=x\n\nport=80").unwrap();
    println!("{} keys, port {}", cfg.len(), cfg["port"]);
    println!("{:?}", parse_all("ok=1\nbroken"));
    println!("divide {}", divide(7, 2));
}

// Example 2: assertions at runtime.
fn demo_asserts() {
    println!("-- assertions --");

    assert!(sum(&[]) == 0);
    assert_eq!(sum(&[2, 2]), 4, "two plus two");
    assert_ne!(sum(&[1]), 0);
    debug_assert!(sum(&[1, 1]) == 2, "only checked in debug builds");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("  panic message: {}", s.lines().next().unwrap_or(""));
        }
    }));

    let _ = std::panic::catch_unwind(|| assert_eq!(1 + 1, 3, "math is broken"));
    let _ = std::panic::catch_unwind(|| divide(1, 0));
    std::panic::set_hook(hook);
}

fn not_ready_yet() -> u32 {
    todo!("pick a storage backend")
}

#[allow(dead_code)]
fn legacy_export() -> Vec<u8> {
    unimplemented!("CSV export was removed in v2")
}

fn sign(n: i32) -> &'static str {
    match n.signum() {
        -1 => "negative",
        0 => "zero",
        1 => "positive",
        _ => unreachable!("signum only returns -1, 0, or 1"),
    }
}

// Example 3: todo!, unimplemented!, unreachable!.
fn demo_placeholders() {
    println!("-- todo! / unreachable! --");

    println!("{} {} {}", sign(-4), sign(0), sign(9));
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let r = std::panic::catch_unwind(not_ready_yet);
    std::panic::set_hook(hook);
    println!("calling todo!() panics: {}", r.is_err());

    // todo!() has type !, so a function body of just todo!() still type-checks.
}

// Example 4: how to run the tests below.
fn demo_running_tests() {
    println!("-- running tests --");

    println!("cargo test --bin 54_tests_docs            # run all tests in this lesson");
    println!(
        "cargo test --bin 54_tests_docs parse      # only tests whose name contains parse"
    );
    println!("cargo test --bin 54_tests_docs -- --ignored       # run #[ignore] tests");
    println!("cargo test --bin 54_tests_docs -- --nocapture     # show println! output");
}

fn main() {
    demo_code();
    demo_asserts();
    demo_placeholders();
    demo_running_tests();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_empty() {
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn sum_some() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_ne!(sum(&[1]), 0);
    }

    #[test]
    fn parse_pair_trims() {
        assert_eq!(parse_pair(" k = v "), Some(("k", "v")));
        assert_eq!(parse_pair("no equals"), None);
    }

    #[test]
    fn parse_all_reports_bad_line() {
        let err = parse_all("a=1\nbad").unwrap_err();
        assert!(err.contains("bad"), "unexpected error: {err}");
    }

    // A test can return Result and use ?.
    #[test]
    fn parse_all_with_question_mark() -> Result<(), String> {
        let m = parse_all("x=1\ny=2")?;
        assert_eq!(m.len(), 2);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics() {
        divide(1, 0);
    }

    #[test]
    #[ignore = "slow; run with --ignored"]
    fn slow_test() {
        assert_eq!((0..10_000_000u64).sum::<u64>(), 49_999_995_000_000);
    }

    // Table-driven tests keep many cases in one place.
    #[test]
    fn sign_table() {
        for (n, want) in [(-5, "negative"), (0, "zero"), (3, "positive")] {
            assert_eq!(sign(n), want, "sign({n})");
        }
    }
}
