// 07. Control flow: if, loop, while, for, ranges, and labels
//
// if takes a bool, with no parentheses around the condition and a block for the body.
// Because if is an expression, every branch must have the same type when you use the
// result.
//
// loop repeats until break, and break can carry a value out. while repeats while a
// condition holds; while let repeats while a pattern matches. for walks anything that
// implements IntoIterator, and ranges such as 0..3 and 0..=3 are the usual counters.
// continue skips to the next round. A label such as 'outer lets break and continue
// target an outer loop.
//
// Run: cargo run --bin 07_control_flow

// Example 1: if, else if, and if as an expression.
fn demo_if() {
    println!("-- if --");

    for n in [-4, 0, 7] {
        if n < 0 {
            println!("{n} neg");
        } else if n == 0 {
            println!("{n} zero");
        } else {
            println!("{n} pos");
        }
    }

    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    let abs = if n < 0 { -n } else { n };
    println!("{parity} abs {abs}");

    // let bad = if n > 0 { 1 } else { "one" };
    // error[E0308]: `if` and `else` have incompatible types

    // let chains join several conditions, including if let, with &&.
    let input = Some(12);
    if let Some(v) = input
        && v > 10
    {
        println!("big value {v}");
    }
}

// Example 2: loop, break with a value, and continue.
fn demo_loop() {
    println!("-- loop --");

    let mut n = 0;
    let found = loop {
        n += 1;
        if n == 3 {
            break n * 10;
        }
    };

    println!("found {found}");

    // Retry until something works.
    let attempts = ["x", "y", "42"];
    let mut i = 0;
    let value: i32 = loop {
        match attempts[i].parse() {
            Ok(v) => break v,
            Err(_) => {
                println!("attempt {i} failed");
                i += 1;
            }
        }
    };

    println!("value {value}");
}

// Example 3: while and while let.
fn demo_while() {
    println!("-- while --");

    let mut n = 0;
    while n < 3 {
        print!("n {n} ");
        n += 1;
    }

    println!();

    // Collatz: an unknown number of steps is a natural fit for while.
    let mut x = 6;
    let mut steps = 0;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        steps += 1;
    }

    println!("collatz steps {steps}");

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("pop {top} ");
    }

    println!();
}

// Example 4: for over collections, with and without borrowing.
fn demo_for() {
    println!("-- for --");

    for i in 0..3 {
        print!("count {i} ");
    }

    println!();

    let mut v = vec!["a", "b", "c"];
    for (i, item) in v.iter().enumerate() {
        print!("{i}:{item} ");
    }

    println!();
    for item in &mut v {
        *item = "z";
    }

    for item in v {
        print!("{item} ");
    }

    println!();

    // v was consumed by the last loop; using it here would be error[E0382].

    for i in (0..10).rev().step_by(3) {
        print!("{i} ");
    }

    println!();

    for n in 1..=10 {
        if n % 3 != 0 {
            continue;
        }

        print!("div3 {n} ");
    }

    println!();
}

// Example 5: ranges are values you can store and pass around.
fn demo_ranges() {
    println!("-- ranges --");

    let r = 1..=3;
    println!("{:?} contains 3? {}", r.clone().collect::<Vec<_>>(), r.contains(&3));

    let a = [10, 20, 30, 40];
    println!("{:?} {:?} {:?}", &a[1..3], &a[..2], &a[2..]);
    println!("{:?}", (10..).take(3).collect::<Vec<_>>());
    println!("len {} empty? {}", (0..5).len(), (5..5).is_empty());
    println!("chars {:?}", ('a'..='e').collect::<String>());

    let total: i32 = (1..=100).sum();
    println!("1..=100 sums to {total}");
}

// Example 6: labeled loops.
fn demo_labels() {
    println!("-- labels --");

    let mut hits = 0;
    'outer: for i in 0..4 {
        for j in 0..4 {
            if i == 2 && j == 1 {
                break 'outer;
            }

            hits += 1;
        }
    }

    println!("hits {hits}");

    // Find the first pair that sums to a target.
    let nums = [3, 8, 1, 6];
    let target = 7;
    let pair = 'search: {
        for (i, a) in nums.iter().enumerate() {
            for b in &nums[i + 1..] {
                if a + b == target {
                    break 'search Some((*a, *b));
                }
            }
        }

        None
    };

    println!("pair {pair:?}");

    'rows: for row in 0..3 {
        for col in 0..3 {
            if col > row {
                continue 'rows;
            }

            print!("({row},{col}) ");
        }
    }

    println!();
}

fn main() {
    demo_if();
    demo_loop();
    demo_while();
    demo_for();
    demo_ranges();
    demo_labels();
}
