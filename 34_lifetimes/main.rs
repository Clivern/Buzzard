// 34. Lifetimes, lifetimes in structs, and elision
//
// A lifetime such as 'a names how long a reference is valid. The compiler already
// checks every reference; annotations do not change how long anything lives. They
// describe relationships the compiler cannot work out from a signature alone, such as
// "the result borrows from x, not from y".
//
// A struct that holds a reference needs a lifetime parameter: struct Excerpt<'a> { s:
// &'a str }. The struct then cannot outlive the data it borrows. This is how iterators
// and parsers hold a borrow of the text they walk.
//
// Elision rules fill in the obvious cases so you rarely write lifetimes: each input
// reference gets its own lifetime; if there is exactly one input lifetime, outputs get
// it; if there is a &self, outputs get self's lifetime. When the rules are not enough,
// the compiler asks you to spell it out.
//
// Run: cargo run --bin 34_lifetimes

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// fn longest_bad(x: &str, y: &str) -> &str { x }
// error[E0106]: missing lifetime specifier

// Example 1: one lifetime tying the inputs to the output.
fn demo_longest() {
    println!("-- longest --");

    let a = String::from("abcd");
    let b = "xyz";
    println!("{}", longest(&a, b));

    let outer = String::from("long string is long");
    let result;
    {
        let inner = String::from("xyz");

        // The result may only be used while both inputs are alive.
        let r = longest(&outer, &inner);
        println!("inside {r}");
        result = longest(&outer, "short");
    }

    println!("outside {result}");

    // let bad;
    // { let inner = String::from("xyz"); bad = longest(&outer, &inner); }
    // println!("{bad}");
    // error[E0597]: `inner` does not live long enough
}

// The result only ever comes from x, so y gets its own unrelated lifetime.
fn first_of<'a>(x: &'a str, _y: &str) -> &'a str {
    x.split(',').next().unwrap_or(x)
}

// Example 2: different lifetimes for different inputs.
fn demo_independent() {
    println!("-- independent lifetimes --");

    let csv = String::from("a,b,c");
    let picked;
    {
        let temp = String::from("ignored");
        picked = first_of(&csv, &temp);
    }

    println!("picked {picked} even though temp is gone");
}

struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    // Elision: the output borrows from self.
    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }

    // Explicit: this output lives as long as the original text, not just &self.
    fn text(&self) -> &'a str {
        self.text
    }
}

// Example 3: structs that hold references.
fn demo_structs() {
    println!("-- structs with references --");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();
    let e = Excerpt { text: first };
    println!("{} / first word {}", e.text, e.first_word());

    let kept: &str = {
        let e2 = Excerpt { text: &novel };
        e2.text() // outlives e2, because it borrows from novel
    };

    println!("kept {}", &kept[..8]);

    // let e;
    // { let s = String::from("temp"); e = Excerpt { text: &s }; }
    // println!("{}", e.text);
    // error[E0597]: `s` does not live long enough
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    fn next_token(&mut self) -> Option<&'a str> {
        let rest = &self.input[self.pos..];
        let rest_trimmed = rest.trim_start();
        if rest_trimmed.is_empty() {
            return None;
        }

        let start = self.pos + (rest.len() - rest_trimmed.len());
        let len = rest_trimmed.find(' ').unwrap_or(rest_trimmed.len());
        self.pos = start + len;
        Some(&self.input[start..start + len])
    }
}

// Example 4: a parser that hands out slices of its input.
fn demo_parser() {
    println!("-- borrowing parser --");

    let src = String::from("let  x = 42");
    let mut p = Parser::new(&src);
    let mut tokens = Vec::new();
    while let Some(t) = p.next_token() {
        tokens.push(t);
    }

    // The tokens borrow src, not the parser, so they outlive p.
    drop(p);
    println!("{tokens:?}");
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}

fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or(s)
}

// Example 5: elision in action.
fn demo_elision() {
    println!("-- elision --");

    // These two signatures mean exactly the same thing.
    println!("{} {}", first_word("hello rust"), first_word_explicit("hi there"));

    // '_ is the anonymous lifetime: "some lifetime, infer it".
    let e: Excerpt<'_> = Excerpt { text: "anonymous lifetime" };
    println!("{}", e.first_word());
}

// Example 6: lifetimes on methods returning one of several borrows.
struct Config {
    name: String,
    fallback: String,
}

impl Config {
    fn pick<'a>(&'a self, override_name: Option<&'a str>) -> &'a str {
        override_name.unwrap_or(if self.name.is_empty() {
            &self.fallback
        } else {
            &self.name
        })
    }
}

fn demo_method_lifetimes() {
    println!("-- method lifetimes --");

    let c = Config { name: String::new(), fallback: "default".into() };
    println!("{} {}", c.pick(None), c.pick(Some("cli")));
}

fn main() {
    demo_longest();
    demo_independent();
    demo_structs();
    demo_parser();
    demo_elision();
    demo_method_lifetimes();
}
