// 02. Variables, mutability, constants, and shadowing
//
// let binds a name to a value. The binding is immutable by default. let mut marks a
// binding you can assign to later. Prefer immutable bindings, and add mut only when the
// later assignment is the point.
//
// The type can be written after a colon, or left for the compiler to infer from how the
// value is used. A second let with the same name shadows the first: the new binding can
// even have a different type. That is not mutation; the old value is just unreachable.
//
// const names a value known at compile time. The type is required and the name is in
// SCREAMING_SNAKE_CASE. const is inlined wherever it is used. static has one fixed
// address for the whole run. type makes an alias, not a new type.
//
// Run: cargo run --bin 02_variables

// Example 1: bindings, patterns, and the _ placeholder.
fn demo_bindings() {
    println!("-- bindings --");

    let count: i32 = 7;
    let inferred = 7;
    let (left, right) = (1, 2);
    let [first, .., last] = [10, 20, 30, 40];
    println!("explicit {count} inferred {inferred} pair {left} {right}");
    println!("first {first} last {last}");

    // A leading underscore silences the unused-variable warning.
    let _unused = 0;

    // A bare _ does not bind at all.
    let _ = count + 1;
}

// Example 2: mut, and declaring before assigning.
fn demo_mutability() {
    println!("-- mutability --");

    let mut y = 23;
    println!("y is {y}");
    y = 50;
    y += 1;
    println!("y is now {y}");

    // let x = 1;
    // x = 2;
    // error[E0384]: cannot assign twice to immutable variable `x`

    // A binding may be declared first and assigned once later, on every path.
    let label;
    if y > 40 {
        label = "big";
    } else {
        label = "small";
    }

    println!("label {label}");
}

// Example 3: shadowing changes the value, the type, or both.
fn demo_shadowing() {
    println!("-- shadowing --");

    let n = 1;
    let n = n + 1;
    println!("as int {n}");

    let n = "two";
    println!("shadowed {n}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces {spaces}");

    // A shadow in an inner block ends with the block.
    let x = 5;
    {
        let x = x * 10;
        println!("inner x {x}");
    }

    println!("outer x {x}");

    // Shadowing is the usual way to parse in steps.
    let input = " 42 ";
    let input = input.trim();
    let input: u32 = input.parse().unwrap();
    println!("parsed {input}");
}

const MAX: i32 = 100;
const LIMITS: [i32; 3] = [MAX / 4, MAX / 2, MAX];
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
static HOST: &str = "localhost";
static GREETING: &str = "hello";

// Example 4: const and static. A const expression is evaluated at compile time.
fn demo_constants() {
    println!("-- const and static --");

    println!("max {MAX} limits {LIMITS:?} day {SECONDS_PER_DAY}s");
    println!("host {HOST} greeting {GREETING}");

    // const is inlined: each use is a fresh copy. static always has the same address.
    let a: *const &str = &GREETING;
    let b: *const &str = &GREETING;
    println!("static same address? {}", std::ptr::eq(a, b));

    // A const can also live inside a function.
    const LOCAL: &str = "local const";
    println!("{LOCAL}");
}

// Example 5: type inference. Many types implement FromStr, so parse needs a hint.
fn demo_inference() {
    println!("-- inference --");

    let n: i32 = "12".parse().unwrap();
    let m = "12".parse::<i64>().unwrap();
    let nums = vec![1, 2, 3];
    let first: Option<_> = nums.into_iter().next();
    println!("{n} {m} {first:?}");

    // let v = Vec::new();
    // error[E0282]: type annotations needed for `Vec<_>`

    // A later use is enough to pin the type.
    let mut later = Vec::new();
    later.push(1u8);
    println!("later {later:?}");
}

type Kilometers = i32;
type Pair<T> = (T, T);

fn fly(distance: Kilometers) {
    println!("flew {distance} km");
}

// Example 6: type aliases. An alias is interchangeable with the real type.
fn demo_aliases() {
    println!("-- aliases --");

    let d: Kilometers = 40;
    let raw: i32 = d;
    fly(raw);

    let p: Pair<&str> = ("left", "right");
    println!("{p:?}");

    // Aliases shorten long types. A newtype struct (lesson 28) is the way to make the
    // compiler reject mixing two meanings of the same type.
}

fn main() {
    demo_bindings();
    demo_mutability();
    demo_shadowing();
    demo_constants();
    demo_inference();
    demo_aliases();
}
