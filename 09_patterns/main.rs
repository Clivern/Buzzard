// 09. Patterns: slices, or, ranges, and references
//
// Patterns appear in match, let, if let, function parameters, and for loops. Beyond
// literals and names, they can match the shape of a slice ([first, rest @ ..]), offer
// alternatives (1 | 2), test a range (1..=5), and look through references (&x, ref x).
//
// A slice pattern is exhaustive only when every length is covered, which usually means
// a final arm with a .. rest. Every branch of an or-pattern must bind the same names.
// Range patterns need constants. Matching on a reference gives you references to the
// fields, so nothing is moved out.
//
// Run: cargo run --bin 09_patterns

fn describe(v: &[i32]) -> String {
    match v {
        [] => "empty".to_string(),
        [x] => format!("one {x}"),
        [a, b] => format!("two {a} {b}"),
        [first, .., last] => format!("many from {first} to {last}"),
    }
}

// Example 1: slice patterns.
fn demo_slice_patterns() {
    println!("-- slice patterns --");

    let v = [1, 2, 3];
    let [first, rest @ ..] = v;
    println!("{first} {rest:?}");

    for s in [&[][..], &[7], &[1, 2], &[1, 2, 3, 4]] {
        println!("{}", describe(s));
    }

    match &v[..] {
        [1, ..] => println!("starts with 1"),
        _ => println!("does not start with 1"),
    }

    let words = ["cargo", "run", "--release"];
    if let ["cargo", cmd, flags @ ..] = words {
        println!("command {cmd} flags {flags:?}");
    }
}

fn sum_recursive(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        [head, tail @ ..] => head + sum_recursive(tail),
    }
}

// Example 2: recursion over a slice, head and tail style.
fn demo_slice_recursion() {
    println!("-- slice recursion --");

    println!("sum {}", sum_recursive(&[1, 2, 3, 4]));
}

// Example 3: or patterns.
fn demo_or_patterns() {
    println!("-- or patterns --");

    for n in [1, 2, 8] {
        match n {
            1 | 2 => println!("{n} small"),
            _ => println!("{n} other"),
        }
    }

    let results: [Result<i32, i32>; 2] = [Ok(3), Err(4)];
    for r in results {
        // Both branches bind x with the same type.
        let (Ok(x) | Err(x)) = r;
        println!("value {x}");
    }

    let cmd = Some("quit");
    if let Some("q" | "quit" | "exit") = cmd {
        println!("bye");
    }
}

// Example 4: range patterns.
fn demo_range_patterns() {
    println!("-- range patterns --");

    for c in ['m', 'Q', '7', '!'] {
        let kind = match c {
            'a'..='z' => "lower",
            'A'..='Z' => "upper",
            '0'..='9' => "digit",
            _ => "other",
        };

        println!("{c} {kind}");
    }

    for score in [95, 81, 40, 120] {
        let grade = match score {
            90..=100 => 'A',
            80..90 => 'B',
            0..80 => 'C',
            _ => '?',
        };

        println!("{score} -> {grade}");
    }
}

struct User {
    name: String,
    age: u32,
}

// Example 5: references in patterns.
fn demo_ref_patterns() {
    println!("-- ref patterns --");

    let s = Some(String::from("hi"));

    // Matching on &s yields &String, so s is still usable afterwards.
    match &s {
        Some(v) => println!("borrowed {v}"),
        None => {}
    }

    println!("still have {s:?}");

    // ref binds by reference in a pattern on an owned value.
    let user = User { name: String::from("ada"), age: 36 };
    let User { ref name, age } = user;
    println!("{name} {age} (user.name still owned by user: {})", user.name);

    // & in a pattern strips a reference from the value.
    let nums = [1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|&n| n * 2).collect();
    println!("{doubled:?}");

    for &(a, b) in &[(1, 'x'), (2, 'y')] {
        print!("{a}{b} ");
    }

    println!();

    // ref mut changes the value in place.
    let mut pair = (1, String::from("a"));
    let (_, ref mut text) = pair;
    text.push('!');
    println!("{pair:?}");
}

// Example 6: patterns in let, fn params, and nested structs.
fn demo_nested() {
    println!("-- nested --");

    struct Point {
        x: i32,
        y: i32,
    }

    struct Segment {
        a: Point,
        b: Point,
    }

    let seg = Segment { a: Point { x: 0, y: 0 }, b: Point { x: 3, y: 4 } };
    let Segment { a: Point { x: x1, y: y1 }, b: Point { x: x2, y: y2 } } = seg;
    println!("dx {} dy {}", x2 - x1, y2 - y1);

    match seg.b {
        Point { x: 0, .. } => println!("b on y axis"),
        Point { x, y: 0..=5 } => println!("b low, x {x}"),
        Point { .. } => println!("b elsewhere"),
    }
}

fn main() {
    demo_slice_patterns();
    demo_slice_recursion();
    demo_or_patterns();
    demo_range_patterns();
    demo_ref_patterns();
    demo_nested();
}
