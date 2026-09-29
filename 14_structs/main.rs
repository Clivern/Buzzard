// 14. Structs: named, tuple, and unit
//
// A struct groups named fields. Point { x: 1, y: 2 } sets them by name, and field init
// shorthand Point { x, y } uses local variables of the same name. Every field must be
// set; there is no implicit zero. Mutability belongs to the binding, not to a field.
//
// ..other in a literal (struct update syntax) copies the remaining fields from another
// instance. It moves the fields that are not Copy.
//
// A tuple struct has unnamed fields: struct Color(u8, u8, u8). It is a distinct type
// from a plain tuple, which makes it the tool for newtypes such as UserId(u64). A unit
// struct has no fields at all, and is useful as a marker or a type that only carries
// trait impls.
//
// Run: cargo run --bin 14_structs

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Example 1: creating, reading, and updating fields.
fn demo_named_structs() {
    println!("-- named structs --");

    let name = String::from("Ada");
    let age = 36;
    let mut a = Person { name, age };
    a.age += 1;
    println!("{a:?}");
    println!("{} is {}", a.name, a.age);

    // let b = Person { name: String::from("x") };
    // error[E0063]: missing field `age` in initializer of `Person`
}

fn new_person(name: &str, age: u32) -> Person {
    Person { name: name.to_string(), age }
}

// Example 2: building structs from functions.
fn demo_constructors() {
    println!("-- constructor fn --");

    let people = [new_person("ann", 30), new_person("ben", 25)];
    for p in &people {
        println!("{} {}", p.name, p.age);
    }

    let oldest = people.iter().max_by_key(|p| p.age).unwrap();
    println!("oldest {}", oldest.name);
}

#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    tls: bool,
    retries: u8,
}

// Example 3: struct update syntax.
fn demo_update() {
    println!("-- struct update --");

    let base =
        Config { host: String::from("localhost"), port: 8080, tls: false, retries: 3 };
    let prod =
        Config { host: String::from("api.example.com"), tls: true, ..base.clone() };
    println!("{prod:?}");
    println!(
        "prod {}:{} tls {} retries {}",
        prod.host, prod.port, prod.tls, prod.retries
    );

    // Without clone, ..base moves base.host if it is not overridden.
    let staging = Config { port: 9090, ..base };
    println!("{staging:?}");
    println!("base.port still readable: {}", base.port);

    // println!("{base:?}"); // error[E0382]: borrow of partially moved value: `base`
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Color(u8, u8, u8);

struct UserId(u64);
struct OrderId(u64);

fn lookup(id: UserId) -> &'static str {
    if id.0 == 42 { "ada" } else { "unknown" }
}

// Example 4: tuple structs and newtypes.
fn demo_tuple_structs() {
    println!("-- tuple structs --");

    let red = Color(255, 0, 0);
    let Color(r, g, b) = red;
    println!("r {} also {r} {g} {b}", red.0);
    println!("equal? {}", red == Color(255, 0, 0));

    println!("user {}", lookup(UserId(42)));
    let order = OrderId(42);

    // lookup(order); // error[E0308]: expected `UserId`, found `OrderId`
    println!("order id {} is not a user id", order.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Meters;

#[derive(Debug)]
struct Ready;

fn start(_token: Ready) {
    println!("started with a Ready token");
}

// Example 5: unit structs.
fn demo_unit_structs() {
    println!("-- unit structs --");

    start(Ready);
    println!("{Ready:?} size {}", size_of::<Ready>());

    let (m1, m2) = (Meters, Meters);
    println!("{m1:?} == {m2:?}? {}", m1 == m2);
}

#[derive(Debug)]
struct Rect {
    origin: (i32, i32),
    size: Size,
}

#[derive(Debug)]
struct Size {
    w: u32,
    h: u32,
}

// Example 6: nesting, destructuring, and references to structs.
fn demo_nesting() {
    println!("-- nesting --");

    let mut r = Rect { origin: (0, 0), size: Size { w: 4, h: 3 } };
    r.size.w *= 2;

    let Rect { origin: (x, y), size: Size { w, h } } = &r;
    println!("at ({x},{y}) {w}x{h} area {}", w * h);

    let size = &mut r.size;
    size.h += 1;
    println!("{r:?}");
}

fn main() {
    demo_named_structs();
    demo_constructors();
    demo_update();
    demo_tuple_structs();
    demo_unit_structs();
    demo_nesting();
}
