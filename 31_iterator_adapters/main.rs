// 31. Iterator adapters and consumers
//
// Adapters take an iterator and return a new, lazy one: map, filter, filter_map, take,
// skip, take_while, enumerate, zip, chain, flatten, flat_map, peekable, inspect, rev,
// step_by, cycle, cloned, copied. Consumers pull the values through and produce a
// result: collect, sum, product, count, fold, reduce, min, max, any, all, find,
// position, for_each.
//
// The usual shape is: describe the work as a chain of adapters, then consume once.
// collect is driven by the type you ask for: Vec, String, HashMap, HashSet, and even
// Result<Vec<_>, E>, which stops at the first error.
//
// Run: cargo run --bin 31_iterator_adapters

use std::collections::{BTreeMap, HashMap, HashSet};

// Example 1: map, filter, and friends.
fn demo_map_filter() {
    println!("-- map / filter --");

    let v: Vec<_> = (0..10).filter(|n| n % 2 == 0).map(|n| n * n).take(3).collect();
    println!("{v:?}");

    let words = ["3", "x", "7", "", "10"];
    let nums: Vec<i32> = words.iter().filter_map(|w| w.parse().ok()).collect();
    println!("filter_map {nums:?}");

    let firsts: Vec<char> =
        ["apple", "", "kiwi"].iter().filter_map(|w| w.chars().next()).collect();
    println!("first chars {firsts:?}");
    println!("skip/step_by {:?}", (0..20).skip(5).step_by(4).collect::<Vec<_>>());
}

// Example 2: take_while, skip_while, map_while.
fn demo_while_adapters() {
    println!("-- take_while / skip_while --");

    let v = [1, 3, 5, 6, 7, 9];
    println!(
        "take_while odd {:?}",
        v.iter().take_while(|n| *n % 2 == 1).collect::<Vec<_>>()
    );
    println!(
        "skip_while odd {:?}",
        v.iter().skip_while(|n| *n % 2 == 1).collect::<Vec<_>>()
    );

    let parsed: Vec<i32> =
        ["1", "2", "x", "4"].iter().map_while(|s| s.parse().ok()).collect();
    println!("map_while stops at first failure {parsed:?}");
}

// Example 3: enumerate, zip, unzip, chain, cycle.
fn demo_combining() {
    println!("-- combining --");

    for (i, c) in "abc".chars().enumerate() {
        print!("{i}{c} ");
    }

    println!();
    let names = ["ann", "ben", "cy"];
    let ages = [30, 25];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("zip stops at the shorter: {pairs:?}");

    let (left, right): (Vec<_>, Vec<_>) = [(1, 'a'), (2, 'b')].into_iter().unzip();
    println!("unzip {left:?} {right:?}");

    let all: Vec<_> = (1..3).chain(7..9).collect();
    println!("chain {all:?}");

    let teams = ["red", "blue"];
    let assigned: Vec<_> = names.iter().zip(teams.iter().cycle()).collect();
    println!("cycle {assigned:?}");
}

// Example 4: flatten and flat_map.
fn demo_flatten() {
    println!("-- flatten / flat_map --");

    let nested = vec![vec![1, 2], vec![], vec![3]];
    println!("flatten {:?}", nested.into_iter().flatten().collect::<Vec<_>>());

    let lines = ["a b", "c d e"];
    let words: Vec<&str> = lines.iter().flat_map(|l| l.split(' ')).collect();
    println!("flat_map {words:?}");

    let opts = [Some(1), None, Some(3)];
    println!("flatten Options {:?}", opts.iter().flatten().collect::<Vec<_>>());

    let doubled: Vec<_> = [1, 2, 3].iter().copied().flat_map(|n| [n, n]).collect();
    println!("{doubled:?}");
}

// Example 5: peekable, inspect, cloned, copied.
fn demo_peek_inspect() {
    println!("-- peekable / inspect --");

    let mut it = [1, 2, 3].into_iter().peekable();
    println!("peek {:?}", it.peek());
    println!("next {:?}", it.next());
    println!("peek {:?}", it.peek());

    // A tiny tokenizer: group runs of digits.
    let mut chars = "12+345-6".chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            let mut num = c.to_string();
            while let Some(d) = chars.next_if(|d| d.is_ascii_digit()) {
                num.push(d);
            }

            tokens.push(num);
        } else {
            tokens.push(c.to_string());
        }
    }

    println!("tokens {tokens:?}");

    let v: Vec<i32> = [1, 2, 3]
        .iter()
        .inspect(|n| print!("see {n} "))
        .copied()
        .filter(|n| n % 2 == 1)
        .collect();

    println!("-> {v:?}");
    let owned: Vec<String> = [String::from("a")].iter().cloned().collect();
    println!("cloned {owned:?}");
}

// Example 6: fold, reduce, scan, and the numeric consumers.
fn demo_fold() {
    println!("-- fold / reduce / scan --");

    let sum = (1..=4).fold(0, |acc, n| acc + n);
    let prod = (1..=4).product::<i32>();
    let max = [3, 9, 1].into_iter().reduce(|a, b| a.max(b));
    println!("{sum} {prod} {max:?}");

    let csv = ["a", "b", "c"].iter().fold(String::new(), |mut acc, s| {
        if !acc.is_empty() {
            acc.push(',');
        }

        acc.push_str(s);
        acc
    });

    println!("fold into string {csv}");
    let running: Vec<_> = [1, 2, 3, 4]
        .iter()
        .scan(0, |acc, &n| {
            *acc += n;
            Some(*acc)
        })
        .collect();

    println!("running totals {running:?}");
    let checked = [100u8, 100, 100].iter().try_fold(0u8, |acc, &n| acc.checked_add(n));
    println!("try_fold with overflow {checked:?}");
}

// Example 7: searching and testing.
fn demo_search() {
    println!("-- search --");

    let v = [4, 8, 15, 16, 23, 42];
    println!(
        "any >40 {} all even {}",
        v.iter().any(|&n| n > 40),
        v.iter().all(|n| n % 2 == 0)
    );
    println!(
        "find {:?} position {:?}",
        v.iter().find(|&&n| n > 10),
        v.iter().position(|&n| n == 16)
    );
    println!(
        "find_map {:?}",
        ["x", "5", "y"].iter().find_map(|s| s.parse::<i32>().ok())
    );
    println!(
        "min {:?} max_by_key {:?}",
        v.iter().min(),
        ["aa", "b", "cccc"].iter().max_by_key(|s| s.len())
    );

    let (even, odd): (Vec<i32>, Vec<i32>) = v.iter().partition(|&&n| n % 2 == 0);
    println!("partition {even:?} {odd:?}");
}

// Example 8: collecting into different targets.
fn demo_collect() {
    println!("-- collect --");

    let set: HashSet<_> = ["a", "a", "b"].into_iter().collect();
    println!("HashSet len {}", set.len());

    let s: String = ['r', 'u', 's', 't'].into_iter().collect();
    println!("String {s}");

    let joined: String =
        ["x", "y"].iter().map(|s| s.to_uppercase()).collect::<Vec<_>>().join("-");
    println!("join {joined}");

    let r: Result<Vec<_>, _> =
        ["1", "2", "x"].iter().map(|t| t.parse::<i32>()).collect();
    println!("Result {r:?}");

    // Pairs collect into maps; with duplicate keys, the later one wins.
    let m: HashMap<_, _> = [("a", 1), ("b", 2), ("a", 3)].into_iter().collect();
    let mut keys: Vec<_> = m.iter().collect();
    keys.sort();
    println!("HashMap {keys:?}");

    let lens: BTreeMap<&str, usize> =
        ["pear", "fig"].iter().map(|w| (*w, w.len())).collect();
    println!("BTreeMap {lens:?}");

    let (keys, vals): (Vec<_>, Vec<_>) = lens.into_iter().unzip();
    println!("unzip a map {keys:?} {vals:?}");
}

fn main() {
    demo_map_filter();
    demo_while_adapters();
    demo_combining();
    demo_flatten();
    demo_peek_inspect();
    demo_fold();
    demo_search();
    demo_collect();
}
