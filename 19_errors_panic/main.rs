// 19. Custom errors, error chains, and panic
//
// A type is an error if it implements std::error::Error, which requires Debug and
// Display. In a library, an enum of error cases is easiest for callers to match on.
// Implementing From<Inner> for your error lets ? convert inner errors automatically.
// source() links an error to the one that caused it, so you can print the whole chain.
//
// In a small binary, Box<dyn Error> is a fine catch-all: any error converts into it
// with ?.
//
// panic! is for bugs: a broken invariant, an impossible state. By default it unwinds
// the thread's stack, running destructors. catch_unwind can stop a panic, but it is not
// a replacement for Result.
//
// Run: cargo run --bin 19_errors_panic

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct EmptyName;

impl fmt::Display for EmptyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "empty name")
    }
}

impl Error for EmptyName {}

fn greet(name: &str) -> Result<String, EmptyName> {
    if name.is_empty() {
        return Err(EmptyName);
    }

    Ok(format!("hello {name}"))
}

// Example 1: the smallest custom error.
fn demo_simple_error() {
    println!("-- simple error --");

    println!("{:?} {:?}", greet("Ada"), greet(""));
    if let Err(e) = greet("") {
        println!("display: {e}");
    }
}

#[derive(Debug)]
enum ConfigError {
    Missing(&'static str),
    BadNumber { key: &'static str, source: ParseIntError },
    OutOfRange(u32),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(k) => write!(f, "missing key {k}"),
            ConfigError::BadNumber { key, .. } => write!(f, "key {key} is not a number"),
            ConfigError::OutOfRange(n) => write!(f, "{n} is out of range"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::BadNumber { source, .. } => Some(source),
            _ => None,
        }
    }
}

fn read_port(value: Option<&str>) -> Result<u32, ConfigError> {
    let text = value.ok_or(ConfigError::Missing("port"))?;
    let n: u32 =
        text.parse().map_err(|e| ConfigError::BadNumber { key: "port", source: e })?;
    if n > 65535 {
        return Err(ConfigError::OutOfRange(n));
    }

    Ok(n)
}

// Example 2: an error enum, matched by the caller.
fn demo_error_enum() {
    println!("-- error enum --");

    for input in [Some("8080"), None, Some("x"), Some("99999")] {
        match read_port(input) {
            Ok(p) => println!("port {p}"),
            Err(ConfigError::Missing(k)) => println!("using default for {k}"),
            Err(e) => println!("error: {e}"),
        }
    }
}

fn print_chain(e: &dyn Error) {
    print!("{e}");
    let mut cur = e.source();
    while let Some(inner) = cur {
        print!(" <- caused by: {inner}");
        cur = inner.source();
    }

    println!();
}

// Example 3: walking the source() chain.
fn demo_chain() {
    println!("-- source chain --");

    if let Err(e) = read_port(Some("12a")) {
        print_chain(&e);
    }
}

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Config(ConfigError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(_) => write!(f, "parse failed"),
            AppError::Config(_) => write!(f, "config failed"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Parse(e) => Some(e),
            AppError::Config(e) => Some(e),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

fn run(workers: &str, port: Option<&str>) -> Result<(u32, u32), AppError> {
    let w: u32 = workers.parse()?; // ParseIntError -> AppError via From
    let p = read_port(port)?; // ConfigError -> AppError via From
    Ok((w, p))
}

// Example 4: From lets ? convert between error types.
fn demo_from_conversion() {
    println!("-- From and ? --");

    println!("{:?}", run("4", Some("80")).map_err(|e| e.to_string()));
    for (w, p) in [("x", Some("80")), ("4", Some("bad"))] {
        if let Err(e) = run(w, p) {
            print_chain(&e);
        }
    }
}

fn load_all(a: &str, b: &str) -> Result<i32, Box<dyn Error>> {
    let x: i32 = a.parse()?;
    if x < 0 {
        return Err("negative input".into()); // &str converts into Box<dyn Error>
    }

    let y = greet(b)?.len() as i32;
    Ok(x + y)
}

// Example 5: Box<dyn Error> as a catch-all.
fn demo_box_dyn() {
    println!("-- Box<dyn Error> --");

    println!("{:?}", load_all("1", "ada").map_err(|e| e.to_string()));
    for (a, b) in [("x", "ada"), ("-1", "ada"), ("1", "")] {
        match load_all(a, b) {
            Ok(v) => println!("ok {v}"),
            Err(e) => println!("error: {e}"),
        }
    }

    // Downcast to recover the concrete type.
    if let Err(e) = load_all("1", "")
        && e.downcast_ref::<EmptyName>().is_some()
    {
        println!("it was EmptyName");
    }
}

// Example 6: panics, unwinding, and when to use them.
fn demo_panic() {
    println!("-- panic --");

    // Silence the default "thread panicked" message for this demo.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let result = std::panic::catch_unwind(|| {
        panic!("boom");
    });

    println!("caught panic! {}", result.is_err());

    let v = vec![1, 2, 3];
    let oob = std::panic::catch_unwind(|| v[10]);
    println!("index out of bounds panics: {}", oob.is_err());

    let msg = std::panic::catch_unwind(|| {
        let n: Option<i32> = None;
        n.expect("must be set")
    });

    if let Err(payload) = msg
        && let Some(s) = payload.downcast_ref::<String>()
    {
        // panic!("literal") carries a &str; a formatted message carries a String.
        println!("payload: {s}");
    }

    std::panic::set_hook(default_hook);

    // unreachable!, todo!, and unimplemented! are panics with fixed messages (lesson 54).
}

fn main() {
    demo_simple_error();
    demo_error_enum();
    demo_chain();
    demo_from_conversion();
    demo_box_dyn();
    demo_panic();
}
