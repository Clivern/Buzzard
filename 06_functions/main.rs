// 06. Functions, expressions, and blocks
//
// fn names a function. Parameters need types; the return type follows ->. A function
// with no -> returns (). Functions can be declared in any order.
//
// Rust is expression-oriented. A block { } produces the value of its last expression,
// and a function body is a block. A semicolon turns an expression into a statement and
// throws the value away, leaving (). return exits early. let is a statement.
//
// ! is the never type, for expressions that never finish: panic!, loop {} with no
// break, process::exit. Never can stand in for any type.
//
// Run: cargo run --bin 06_functions

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn swap(a: String, b: String) -> (String, String) {
    (b, a)
}

fn log(msg: &str) {
    println!("log: {msg}");
}

// Example 1: parameters, return types, and ().
fn demo_basics() {
    println!("-- basics --");

    println!("{}", add(2, 3));
    let (left, right) = swap(String::from("left"), String::from("right"));
    println!("{left} {right}");

    let nothing: () = log("returns unit");
    println!("unit {nothing:?}");
}

fn head((x, _): (i32, i32)) -> i32 {
    x
}

fn bump_copy(mut n: i32) -> i32 {
    n += 1;
    n
}

fn bump_in_place(n: &mut i32) {
    *n += 1;
}

fn total(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// Example 2: patterns, mut, and references in parameters.
fn demo_params() {
    println!("-- params --");

    println!("head {}", head((9, 1)));

    // mut on a parameter changes the local copy only.
    let n = 4;
    println!("copy {} original {n}", bump_copy(n));

    // &mut lets the function change the caller's value.
    let mut m = 4;
    bump_in_place(&mut m);
    println!("in place {m}");

    // A slice parameter accepts arrays and Vecs.
    println!("totals {} {}", total(&[1, 2, 3]), total(&vec![10, 20]));
}

fn first(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }

    Some(nums[0])
}

fn classify(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }

    if n == 0 { "zero" } else { "positive" }
}

// Example 3: early return and the tail expression.
fn demo_returns() {
    println!("-- returns --");

    println!("{:?} {:?}", first(&[8, 1]), first(&[]));
    println!("{} {} {}", classify(-3), classify(0), classify(5));

    // fn broken() -> i32 { 5; }
    // error[E0308]: mismatched types (the semicolon made the body return ())
}

// Example 4: blocks, if, and match are all expressions.
fn demo_expressions() {
    println!("-- expressions --");

    let n = 3;
    let label = if n > 2 { "big" } else { "small" };
    let doubled = {
        let x = n;
        x * 2
    };

    let word = match n {
        1 => "one",
        3 => "three",
        _ => "many",
    };

    println!("{label} {doubled} {word}");

    // A block that ends with a semicolon has the value ().
    let unit = {
        let _ = n + 1;
    };

    println!("unit {unit:?}");
}

// Example 5: scope and drop order.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn demo_scope() {
    println!("-- scope --");

    let outer = 1;
    let _a = Noisy("a");
    {
        let inner = 2;
        let _b = Noisy("b");
        println!("inner {inner} outer {outer}");
    }

    let _c = Noisy("c");
    println!("end of demo_scope; c then a drop next, reverse order");
}

fn halt(msg: &str) -> ! {
    panic!("{msg}");
}

fn pick(flag: bool) -> i32 {
    if flag { 7 } else { halt("flag was false") }
}

// Example 6: the never type fits anywhere a value is expected.
fn demo_never() {
    println!("-- never --");

    println!("{}", pick(true));
    let input = ["1", "x", "3"];
    let mut sum = 0;
    for s in input {
        // continue has type !, so this match still produces an i32.
        let n: i32 = match s.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        sum += n;
    }

    println!("sum of valid {sum}");
}

// Example 7: nested functions and function values.
fn demo_function_values() {
    println!("-- function values --");

    fn square(n: i32) -> i32 {
        n * n
    }

    let f: fn(i32, i32) -> i32 = add;
    let ops: [(&str, fn(i32) -> i32); 2] = [("square", square), ("bump", bump_copy)];
    println!("f(2, 3) = {}", f(2, 3));

    for (name, op) in ops {
        println!("{name}(5) = {}", op(5));
    }
}

fn main() {
    demo_basics();
    demo_params();
    demo_returns();
    demo_expressions();
    demo_scope();
    demo_never();
    demo_function_values();
}
