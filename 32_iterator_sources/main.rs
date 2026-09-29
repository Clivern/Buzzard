// 32. Iterator sources: from_fn, successors, once, repeat, empty
//
// You do not always need a struct to make an iterator. std::iter has small building
// blocks:
//
// - from_fn(closure): calls the closure each time; it returns Some to yield, None to stop.
// - successors(seed, f): each item is computed from the previous one.
// - once(v) yields one value; once_with(f) computes it lazily.
// - repeat(v) clones v forever; repeat_n(v, n) stops after n; repeat_with(f) calls f.
// - empty() yields nothing.
//
// The infinite ones need a take or take_while downstream. Combined with chain and zip
// they cover most cases where you would otherwise write a one-off Iterator impl.
//
// Run: cargo run --bin 32_iterator_sources

use std::iter::{
    empty, from_fn, once, once_with, repeat, repeat_n, repeat_with, successors,
};

// Example 1: from_fn with captured state.
fn demo_from_fn() {
    println!("-- from_fn --");

    let mut n = 0;
    let v: Vec<_> = from_fn(|| {
        n += 1;
        if n <= 3 { Some(n) } else { None }
    })
    .collect();
    println!("{v:?}");

    // Read "records" from a buffer until it runs out.
    let mut data = "id=1;id=2;id=3".split(';');
    let ids: Vec<u32> =
        from_fn(|| data.next()?.strip_prefix("id=")?.parse().ok()).collect();
    println!("ids {ids:?}");
}

// Example 2: successors.
fn demo_successors() {
    println!("-- successors --");

    let powers: Vec<_> = successors(Some(1i32), |n| n.checked_mul(2)).take(6).collect();
    println!("powers {powers:?}");

    let all_u8_powers: Vec<u8> = successors(Some(1u8), |n| n.checked_mul(2)).collect();
    println!("u8 powers until overflow {all_u8_powers:?}");

    let collatz: Vec<u64> = successors(Some(6u64), |&n| {
        (n != 1).then(|| if n % 2 == 0 { n / 2 } else { 3 * n + 1 })
    })
    .collect();
    println!("collatz {collatz:?}");

    let digits: Vec<u32> = successors(Some(9045u32), |n| (*n >= 10).then_some(n / 10))
        .map(|n| n % 10)
        .collect();

    println!("digits reversed {digits:?}");
}

// Example 3: once, once_with, and empty.
fn demo_once_empty() {
    println!("-- once / empty --");

    let v: Vec<_> = once(1).chain(once(2)).chain(empty()).collect();
    println!("{v:?}");

    // Add a header row to a list without allocating a new Vec first.
    let rows = ["a", "b"];
    let with_header: Vec<_> = once("NAME").chain(rows).collect();
    println!("{with_header:?}");

    let lazy = once_with(|| {
        println!("  computing the one value");
        42
    });

    println!("before pulling");
    println!("got {:?}", lazy.collect::<Vec<_>>());

    let none: Vec<i32> = empty().collect();
    println!("empty {none:?}");
}

// Example 4: repeat, repeat_n, repeat_with.
fn demo_repeat() {
    println!("-- repeat --");

    println!("{:?}", repeat("x").take(3).collect::<Vec<_>>());
    println!("{:?}", repeat_n(0u8, 4).collect::<Vec<_>>());

    let line: String = repeat('=').take(10).collect();
    println!("{line}");

    let mut seed = 7u32;
    let pseudo: Vec<u32> = repeat_with(|| {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        (seed >> 16) % 100
    })
    .take(5)
    .collect();
    println!("pseudo random {pseudo:?}");
}

// Example 5: returning sources from functions.
fn evens() -> impl Iterator<Item = u32> {
    (0..).step_by(2)
}

fn countdown(from: u32) -> impl Iterator<Item = u32> {
    successors(Some(from), |&n| n.checked_sub(1))
}

fn padded<'a>(items: &'a [&'a str], width: usize) -> impl Iterator<Item = &'a str> + 'a {
    items.iter().copied().chain(repeat("-")).take(width)
}

fn demo_returned_sources() {
    println!("-- returned sources --");

    println!("{:?}", evens().take(5).collect::<Vec<_>>());
    println!("{:?}", countdown(3).collect::<Vec<_>>());
    println!("{:?}", padded(&["a", "b"], 5).collect::<Vec<_>>());
}

// Example 6: windows over generated values.
fn demo_pairs() {
    println!("-- pairs from sources --");

    let squares = (1..).map(|n: u32| n * n);
    let gaps: Vec<_> =
        squares.clone().zip(squares.skip(1)).map(|(a, b)| b - a).take(5).collect();
    println!("gaps between squares {gaps:?}");

    let alternating: Vec<i32> =
        [1, -1].iter().copied().cycle().zip(1..=6).map(|(s, n)| s * n).collect();
    println!("alternating {alternating:?}");
}

fn main() {
    demo_from_fn();
    demo_successors();
    demo_once_empty();
    demo_repeat();
    demo_returned_sources();
    demo_pairs();
}
