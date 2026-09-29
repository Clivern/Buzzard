// 55. Declarative macros with macro_rules!
//
// macro_rules! defines a macro by pattern matching on tokens. Each rule is
// (matcher) => { expansion }. Matchers capture fragments: $x:expr for an expression,
// $t:ty a type, $i:ident a name, $l:literal a literal, $p:pat a pattern, $b:block a
// block, and $tt:tt any single token tree. $( ... ),* repeats zero or more times with a
// comma separator; + means one or more and ? means optional.
//
// Macros expand before type checking, which is why println! can take any number of
// arguments of any type. They are hygienic: a variable created inside a macro does not
// clash with one of the same name at the call site. Rules are tried top to bottom, and
// a macro may call itself recursively.
//
// Prefer a function when a function will do. Reach for a macro for syntax a function
// cannot express: variadic arguments, generating items, or taking code as input.
//
// Run: cargo run --bin 55_macros

use std::collections::HashMap;

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! square_safe {
    ($x:expr) => {{
        let v = $x;
        v * v
    }};
}

// Example 1: simple expression macros, and why expr captures matter.
fn demo_basics() {
    println!("-- basics --");

    println!("{} {}", add!(2, 3), add!(10, 20));

    // $x:expr is captured as one expression, so square!(1 + 2) is (1 + 2) * (1 + 2).
    println!("square!(1 + 2) = {}", square!(1 + 2));

    let mut n = 0;
    let mut next = || {
        n += 1;
        n
    };

    // square! evaluates its argument twice; square_safe! once.
    println!(
        "square!(next()) = {} square_safe!(next()) = {}",
        square!(next()),
        square_safe!(next())
    );
}

macro_rules! vec_of {
    ($($x:expr),* $(,)?) => {{
        #[allow(unused_mut)] // vec_of![] never pushes
        let mut v = Vec::new();
        $(v.push($x);)*
        v
    }};
}

macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}

macro_rules! max_of {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = max_of!($($rest),+);
        if a > b { a } else { b }
    }};
}

// Example 2: repetition and recursion.
fn demo_repetition() {
    println!("-- repetition --");

    println!("{:?} {:?}", vec_of![1, 2, 3], vec_of![4, 5,]);
    let empty: Vec<i32> = vec_of![];
    println!("empty {empty:?}");
    let m = hashmap! { "a" => 1, "b" => 2 };
    let mut pairs: Vec<_> = m.into_iter().collect();
    pairs.sort();
    println!("{pairs:?}");
    println!("max_of {}", max_of!(3, 9, 4, 1));
}

macro_rules! calc {
    (add $a:literal $b:literal) => {
        $a + $b
    };
    (mul $a:literal $b:literal) => {
        $a * $b
    };
    (neg $a:literal) => {
        -$a
    };
}

// Example 3: several rules, matched on literal tokens.
fn demo_rules() {
    println!("-- multiple rules --");

    println!("{} {} {}", calc!(add 2 3), calc!(mul 4 5), calc!(neg 7));

    // calc!(div 1 2); // error: no rules expected the token `div`
}

macro_rules! getters {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Default)]
        struct $name {
            $($field: $ty),*
        }

        impl $name {
            $(fn $field(&self) -> &$ty { &self.$field })*
        }
    };
}

getters!(Profile { name: String, age: u32 });

macro_rules! impl_describe {
    ($($t:ty),*) => {
        $(impl Describe for $t {
            fn describe(&self) -> String {
                format!("{} = {:?}", stringify!($t), self)
            }
        })*
    };
}

trait Describe {
    fn describe(&self) -> String;
}

impl_describe!(i32, bool, &str);

// Example 4: generating items: structs, methods, and trait impls.
fn demo_items() {
    println!("-- generating items --");

    let p = Profile { name: "ada".into(), age: 36 };
    println!("{} {} {:?}", p.name(), p.age(), Profile::default());
    println!("{} | {} | {}", 5.describe(), true.describe(), "hi".describe());
}

macro_rules! time_it {
    ($label:literal, $body:block) => {{
        let start = std::time::Instant::now();
        let out = $body;
        println!("  {} took under 1s: {}", $label, start.elapsed().as_secs() < 1);
        out
    }};
}

macro_rules! swap_hygiene {
    ($a:ident, $b:ident) => {
        let tmp = $a;
        $a = $b;
        $b = tmp;
    };
}

// Example 5: taking a block, and hygiene.
fn demo_blocks_hygiene() {
    println!("-- blocks and hygiene --");

    let total = time_it!("summing", { (1..=100).sum::<i32>() });

    println!("total {total}");
    let tmp = "outer tmp";
    let mut x = 1;
    let mut y = 2;
    swap_hygiene!(x, y);

    // The macro's `tmp` did not overwrite ours.
    println!("x {x} y {y} tmp {tmp:?}");
}

// Example 6: useful built-in macros.
fn demo_builtins() {
    println!("-- built-in macros --");

    println!("stringify!: {}", stringify!(a + b * 2));
    println!("concat!: {}", concat!("v", 1, ".", 2, '-', true));
    println!("line {} column {} file {}", line!(), column!(), file!());
    println!("module_path {}", module_path!());
    println!("matches! {}", matches!(Some(3), Some(n) if n > 2));

    let v = vec![1; 3];
    println!("vec! {v:?} format! {}", format!("{:>4}", 7));

    // compile_error!("stops the build with this message");
}

fn main() {
    demo_basics();
    demo_repetition();
    demo_rules();
    demo_items();
    demo_blocks_hygiene();
    demo_builtins();
}
