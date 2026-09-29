// 16. Enums
//
// An enum is a value that is exactly one of several variants. Unlike a C enum, each
// variant can carry its own data: no data, a tuple, or named fields. match (or if let)
// is how you read that data back, and the compiler makes sure you handle every variant.
//
// An impl on an enum works just like one on a struct; methods usually match on self.
// Fieldless enums can have explicit discriminants and be cast to integers with as.
// Enums are how Rust models "one of these" state, instead of flags and nullable fields.
//
// Run: cargo run --bin 16_enums

#[derive(Debug)]
enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Example 1: variants that carry data.
fn demo_data_variants() {
    println!("-- data variants --");

    let home = Ip::V4(127, 0, 0, 1);
    let loopback = Ip::V6(String::from("::1"));
    for ip in [&home, &loopback] {
        match ip {
            Ip::V4(a, b, c, d) => println!("v4 {a}.{b}.{c}.{d}"),
            Ip::V6(s) => println!("v6 {s}"),
        }
    }

    println!("{home:?}");
}

#[derive(Debug)]
enum Msg {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
    Color(u8, u8, u8),
}

impl Msg {
    fn summary(&self) -> String {
        match self {
            Msg::Quit => String::from("quit"),
            Msg::Move { x, y } => format!("move to {x},{y}"),
            Msg::Text(s) => format!("text:{s}"),
            Msg::Color(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"),
        }
    }

    fn is_quit(&self) -> bool {
        matches!(self, Msg::Quit)
    }
}

// Example 2: methods on enums.
fn demo_enum_methods() {
    println!("-- enum methods --");

    let msgs = vec![
        Msg::Move { x: 1, y: 2 },
        Msg::Text(String::from("hi")),
        Msg::Color(255, 128, 0),
        Msg::Quit,
    ];

    for m in &msgs {
        println!("{} quit? {}", m.summary(), m.is_quit());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    Low = 1,
    Mid = 5,
    High = 10,
}

impl Level {
    fn from_score(n: u32) -> Self {
        match n {
            0..=3 => Level::Low,
            4..=7 => Level::Mid,
            _ => Level::High,
        }
    }
}

// Example 3: fieldless enums and discriminants.
fn demo_discriminants() {
    println!("-- discriminants --");

    println!(
        "{:?}={} {:?}={} {:?}={}",
        Level::Low,
        Level::Low as i32,
        Level::Mid,
        Level::Mid as i32,
        Level::High,
        Level::High as i32
    );

    let levels: Vec<Level> = [2, 6, 9].into_iter().map(Level::from_score).collect();
    println!("{levels:?}");
    println!("size of Level {}", size_of::<Level>());
}

// Example 4: enums as a state machine.
#[derive(Debug)]
enum Door {
    Open,
    Closed,
    Locked { code: u32 },
}

impl Door {
    fn next(self, action: &str) -> Door {
        match (self, action) {
            (Door::Open, "close") => Door::Closed,
            (Door::Closed, "open") => Door::Open,
            (Door::Closed, "lock") => Door::Locked { code: 1234 },
            (Door::Locked { code }, "unlock") if code == 1234 => Door::Closed,
            (state, _) => state,
        }
    }
}

fn demo_state_machine() {
    println!("-- state machine --");

    let mut door = Door::Open;
    for action in ["close", "lock", "open", "unlock", "open"] {
        door = door.next(action);
        println!("after {action:<6} -> {door:?}");
    }
}

// Example 5: recursive enums need a Box.
#[derive(Debug)]
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(a) => -eval(a),
    }
}

fn demo_recursive() {
    println!("-- recursive enum --");

    // (2 + 3) * -4
    let e = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))),
        Box::new(Expr::Neg(Box::new(Expr::Num(4)))),
    );
    println!("{} = {e:?}", eval(&e));

    // enum Bad { Leaf, Node(Bad) }
    // error[E0072]: recursive type `Bad` has infinite size
}

// Example 6: the size of an enum is its largest variant plus a tag.
fn demo_sizes() {
    println!("-- sizes --");

    println!("Msg {} bytes, String {} bytes", size_of::<Msg>(), size_of::<String>());
    println!(
        "Option<Box<i32>> {} vs Box<i32> {}",
        size_of::<Option<Box<i32>>>(),
        size_of::<Box<i32>>()
    );
}

fn main() {
    demo_data_variants();
    demo_enum_methods();
    demo_discriminants();
    demo_state_machine();
    demo_recursive();
    demo_sizes();
}
