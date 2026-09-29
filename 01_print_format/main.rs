// 01. Hello, printing, and formatting
//
// An executable is a binary crate with a function named main. That function is where the
// program starts. println! is a macro: the ! marks it. It writes a line to stdout.
// eprintln! writes to stderr: use stdout for the result and stderr for diagnostics.
//
// Every print macro takes a format string. {} uses Display (for users), {:?} uses Debug
// (for programmers), and {:#?} pretty-prints Debug. Names in braces capture variables:
// {name}. After the colon comes a spec: width, alignment, fill, precision, and radix.
// format! builds a String instead of printing, and write! formats into any writer.
//
// Comments: // runs to the end of the line, /* */ can span lines, /// documents the next
// item, and //! documents the enclosing module. cargo doc renders the doc comments.
//
// Run: cargo run --bin 01_print_format

use std::fmt::Write as _;

// Example 1: the smallest program, and the two output streams.
fn demo_hello() {
    println!("-- hello --");

    println!("Hello, Rust");
    eprintln!("hello on stderr");
    print!("print! adds no newline, ");
    println!("so this continues the line");
}

/// Adds one. Doc comments like this one become HTML with cargo doc.
fn bump(n: i32) -> i32 {
    n + 1
}

// Example 2: the kinds of comments.
fn demo_comments() {
    println!("-- comments --");

    // A line comment.
    let n = /* a block comment can sit inside an expression */ bump(3);
    /*
     * Block comments can span
     * several lines.
     */
    println!("bump(3) = {n}");
}

// Example 3: positional, numbered, named, and captured arguments.
fn demo_placeholders() {
    println!("-- placeholders --");

    let name = "Ada";
    let year = 1843;
    println!("{} wrote notes in {}", name, year);
    println!("{0} and {0} again, then {1}", name, year);
    println!("{who} in {when}", who = name, when = year);
    println!("{name} in {year}");
    println!("literal braces: {{ and }}");
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

// Example 4: Debug, pretty Debug, and dbg!.
fn demo_debug() {
    println!("-- debug --");

    let p = Point { x: 3, y: 4 };
    println!("{p:?}");
    println!("{:?} {:?} {:?}", (1, "two"), [1, 2, 3], Some(5));

    let line = Line { from: Point { x: 0, y: 0 }, to: p };
    println!("{line:#?}");

    // dbg! prints file, line, and the expression to stderr, then returns the value.
    let n = dbg!(line.to.x * 2) + 1;
    println!("n {n} from y {}", line.from.y);
}

// Example 5: width, alignment, and fill.
fn demo_width_align() {
    println!("-- width and align --");

    println!("[{:5}] [{:<5}] [{:^5}] [{:>5}]", "ab", "ab", "ab", "ab");
    println!("[{:*^9}] [{:->6}] [{:_<6}]", "mid", 42, 7);

    let width = 8;
    println!("[{:>width$}] [{:>1$}]", "var", 6);

    for (item, price) in [("tea", 3.5), ("cake", 12.25)] {
        println!("{item:<6}|{price:>8.2}");
    }
}

// Example 6: numbers: padding, precision, sign, and radix.
fn demo_numbers() {
    println!("-- numbers --");

    println!("{:08} {:+} {:.2} {:.0}", 42, 5, 1.23456, 2.5);
    println!("hex {:x} {:X} {:#x} bin {:b} {:#010b} oct {:o}", 255, 255, 255, 5, 5, 8);
    println!("exp {:e} {:E}", 1234.5, 0.00012);

    let pi = std::f64::consts::PI;
    let places = 4;
    println!("pi to {places} places {pi:.places$}");
}

// Example 7: format! builds a String, write! appends to one.
fn demo_format_to_string() {
    println!("-- format! and write! --");

    let id = format!("id={:04}", 7);
    let mut report = String::new();
    write!(report, "{id}").unwrap();

    for n in 1..=3 {
        write!(report, " [{n}]").unwrap();
    }

    writeln!(report, " done").unwrap();
    print!("{report}");
    println!("len {}", report.len());
}

fn main() {
    demo_hello();
    demo_comments();
    demo_placeholders();
    demo_debug();
    demo_width_align();
    demo_numbers();
    demo_format_to_string();
}
