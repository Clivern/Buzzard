// 08. match, if let, let else, and matches!
//
// match compares a value against patterns, top to bottom, and runs the first arm that
// fits. It must be exhaustive: every possible value has an arm, or you add _. match is
// an expression, so every arm produces the same type.
//
// A guard (if after a pattern) adds a condition. @ binds a name to a value while still
// testing it against a subpattern. if let is a one-arm match, while let is its loop
// form, and let else binds on success or diverges on failure. matches! turns a pattern
// into a bool.
//
// Run: cargo run --bin 08_match

// Example 1: matching literals, and match as an expression.
fn demo_match_basics() {
    println!("-- match basics --");

    for n in [1, 2, 3, 9] {
        let word = match n {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "other",
        };

        print!("{word} ");
    }

    println!();

    // let n = 5u8;
    // match n { 0 => {}, 1..=100 => {} }
    // error[E0004]: non-exhaustive patterns: `101_u8..=u8::MAX` not covered

    let pair = (0, -2);
    match pair {
        (0, y) => println!("on the y axis at {y}"),
        (x, 0) => println!("on the x axis at {x}"),
        _ => println!("somewhere else"),
    }
}

// Example 2: guards.
fn demo_guards() {
    println!("-- guards --");

    for n in [-3, 8, 7] {
        match n {
            x if x < 0 => println!("{x} neg"),
            x if x % 2 == 0 => println!("{x} even"),
            x => println!("{x} odd"),
        }
    }

    let point = (3, 3);
    match point {
        (x, y) if x == y => println!("on the diagonal"),
        (x, y) if x + y == 0 => println!("on the anti-diagonal"),
        _ => println!("off both"),
    }
}

// Example 3: @ bindings.
fn demo_bindings() {
    println!("-- @ bindings --");

    for n in [3, 15, 42] {
        match n {
            k @ 1..=5 => println!("small {k}"),
            k @ (10 | 15 | 20) => println!("round-ish {k}"),
            k => println!("other {k}"),
        }
    }

    let msg = Some(7);
    if let Some(n @ 5..) = msg {
        println!("at least five: {n}");
    }
}

#[derive(Debug)]
enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    Dot,
}

// Example 4: destructuring enums and structs in arms.
fn demo_destructure() {
    println!("-- destructure --");

    let shapes = [Shape::Circle { r: 1.0 }, Shape::Rect { w: 2.0, h: 3.0 }, Shape::Dot];
    for s in &shapes {
        let area = match s {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } if w == h => w * w,
            Shape::Rect { w, h } => w * h,
            Shape::Dot => 0.0,
        };

        println!("{s:?} area {area:.2}");
    }
}

// Example 5: if let and if let ... else.
fn demo_if_let() {
    println!("-- if let --");

    let maybe = Some("hi");
    if let Some(msg) = maybe {
        println!("{msg}");
    } else {
        println!("nothing");
    }

    let parsed: Result<i32, _> = "12".parse::<i32>();
    if let Ok(n) = parsed
        && n > 10
    {
        println!("parsed a big number {n}");
    }

    let mut stack = vec![1, 2];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }

    println!();
}

fn greet(name: Option<&str>) {
    let Some(name) = name else {
        println!("no name");
        return;
    };

    println!("hello {name}");
}

fn parse_pair(s: &str) -> Option<(i32, i32)> {
    let Some((a, b)) = s.split_once(',') else {
        return None;
    };

    let (Ok(a), Ok(b)) = (a.trim().parse(), b.trim().parse()) else {
        return None;
    };

    Some((a, b))
}

// Example 6: let else keeps the happy path unindented.
fn demo_let_else() {
    println!("-- let else --");

    greet(Some("Ada"));
    greet(None);
    println!("{:?} {:?} {:?}", parse_pair("3, 4"), parse_pair("3"), parse_pair("a,b"));

    // let Some(x) = Some(1);
    // error[E0005]: refutable pattern in local binding (add an else block)
}

// Example 7: matches! for a quick bool.
fn demo_matches_macro() {
    println!("-- matches! --");

    let n = Some(3);
    println!("{}", matches!(n, Some(x) if x > 0));

    let v = vec![Ok(1), Err("e"), Ok(2)];
    let oks: Vec<_> = v.into_iter().filter(|r| matches!(r, Ok(_))).collect();
    println!("{oks:?}");

    let c = 'q';
    println!("is vowel? {}", matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));

    let shape = Shape::Rect { w: 1.0, h: 1.0 };
    println!("is rect? {}", matches!(shape, Shape::Rect { .. }));
}

fn main() {
    demo_match_basics();
    demo_guards();
    demo_bindings();
    demo_destructure();
    demo_if_let();
    demo_let_else();
    demo_matches_macro();
}
