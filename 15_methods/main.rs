// 15. Methods and associated functions
//
// An impl block attaches functions to a type. A method takes self in one of three
// forms: &self borrows, &mut self borrows mutably, and self takes ownership. Call it
// with a dot, c.inc(); the compiler adds the & or &mut for you (auto-referencing).
//
// A function in an impl with no self is an associated function, called with the type
// name: Counter::new(). That is the usual constructor. Inside an impl, Self is an alias
// for the type. A type can have several impl blocks, and associated consts too.
//
// Run: cargo run --bin 15_methods

#[derive(Debug)]
struct Counter {
    n: i32,
    step: i32,
}

impl Counter {
    const DEFAULT_STEP: i32 = 1;

    fn new() -> Self {
        Self { n: 0, step: Self::DEFAULT_STEP }
    }

    fn with_step(step: i32) -> Self {
        Self { n: 0, step }
    }

    fn value(&self) -> i32 {
        self.n
    }

    fn inc(&mut self) {
        self.n += self.step;
    }

    fn reset(&mut self) -> i32 {
        std::mem::replace(&mut self.n, 0)
    }

    fn into_value(self) -> i32 {
        self.n
    }
}

// Example 1: the three kinds of receivers.
fn demo_receivers() {
    println!("-- receivers --");

    let mut c = Counter::new();
    c.inc();
    c.inc();
    println!("value {}", c.value());
    println!("reset returned {} now {}", c.reset(), c.value());
    c.inc();

    let n = c.into_value();
    println!("into_value {n}");

    // c.value(); // error[E0382]: borrow of moved value: `c`
}

// Example 2: associated functions and constants.
fn demo_associated() {
    println!("-- associated --");

    let mut by_five = Counter::with_step(5);
    by_five.inc();
    println!("{by_five:?} default step {}", Counter::DEFAULT_STEP);

    // A method can be called with its full path too.
    println!("path call {}", Counter::value(&by_five));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    w: f64,
    h: f64,
}

impl Rect {
    fn square(side: f64) -> Self {
        Self { w: side, h: side }
    }

    fn area(&self) -> f64 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }

    fn scaled(self, k: f64) -> Self {
        Self { w: self.w * k, h: self.h * k }
    }
}

// Example 3: methods that take other values.
fn demo_more_params() {
    println!("-- more params --");

    let big = Rect { w: 10.0, h: 8.0 };
    let small = Rect::square(3.0);
    println!("areas {} {}", big.area(), small.area());
    println!(
        "big holds small? {} reverse? {}",
        big.can_hold(&small),
        small.can_hold(&big)
    );

    // Rect is Copy, so scaled(self) does not consume `small`.
    println!("scaled {:?} original {:?}", small.scaled(2.0), small);
}

// Example 4: chaining builder-style methods.
#[derive(Debug, Default)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
}

impl Request {
    fn get(url: &str) -> Self {
        Self { url: url.into(), method: "GET".into(), ..Default::default() }
    }

    fn header(mut self, k: &str, v: &str) -> Self {
        self.headers.push((k.into(), v.into()));
        self
    }

    fn method(mut self, m: &str) -> Self {
        self.method = m.into();
        self
    }
}

fn demo_chaining() {
    println!("-- chaining --");

    let req =
        Request::get("/api").header("accept", "json").header("x-id", "7").method("POST");
    println!("{} {} {:?}", req.method, req.url, req.headers);
}

// Example 5: auto-referencing and auto-dereferencing.
fn demo_auto_ref() {
    println!("-- auto ref --");

    let r = Rect::square(2.0);
    let rr = &r;
    let boxed = Box::new(r);

    // All three calls resolve to Rect::area(&Rect).
    println!("{} {} {}", r.area(), rr.area(), boxed.area());

    let mut c = Counter::new();
    let handle = &mut c;
    handle.inc(); // (*handle).inc()
    println!("through &mut {}", c.value());
}

// Example 6: several impl blocks, and getters that protect a field.
mod temp {
    pub struct Celsius {
        degrees: f64,
    }

    impl Celsius {
        pub fn new(degrees: f64) -> Option<Self> {
            (degrees >= -273.15).then_some(Self { degrees })
        }

        pub fn degrees(&self) -> f64 {
            self.degrees
        }
    }

    impl Celsius {
        pub fn to_fahrenheit(&self) -> f64 {
            self.degrees * 9.0 / 5.0 + 32.0
        }
    }
}

fn demo_getters() {
    println!("-- getters --");

    let t = temp::Celsius::new(100.0).unwrap();
    println!("{} C = {} F", t.degrees(), t.to_fahrenheit());
    println!(
        "below absolute zero is rejected: {}",
        temp::Celsius::new(-300.0).is_none()
    );
}

fn main() {
    demo_receivers();
    demo_associated();
    demo_more_params();
    demo_chaining();
    demo_auto_ref();
    demo_getters();
}
