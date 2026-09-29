// 57. Type-level tools: PhantomData, Sized, Any, and type_name
//
// PhantomData<T> is a zero-sized field that says "this type logically involves T"
// without storing a T. It lets you tag a type with a marker: Id<User> and Id<Team> are
// different types even though both just hold a u64. The same trick builds "typestate"
// APIs, where invalid method calls are compile errors.
//
// Every generic parameter is Sized by default: its size is known at compile time. ?Sized
// opts out so the parameter can also be str, [T], or dyn Trait, which must then live
// behind a pointer (&T, Box<T>).
//
// dyn Any is a type-erased value that can be downcast back with downcast_ref::<T>(). It
// is how panic payloads and some plugin systems carry arbitrary values. TypeId is the
// runtime identity Any uses, and type_name gives a readable name for debugging, but not
// a stable one.
//
// Run: cargo run --bin 57_type_level

use std::any::{Any, TypeId, type_name, type_name_of_val};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

struct Id<T> {
    n: u64,
    _ty: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(n: u64) -> Self {
        Id { n, _ty: PhantomData }
    }
}

struct User;
struct Team;

fn load_user(id: Id<User>) -> String {
    format!("user #{}", id.n)
}

// Example 1: typed ids with PhantomData.
fn demo_typed_ids() {
    println!("-- typed ids --");

    let u = Id::<User>::new(1);
    let t = Id::<Team>::new(1);
    println!("{}", load_user(u));

    // load_user(t); // error[E0308]: expected `Id<User>`, found `Id<Team>`
    println!("team id {} size of Id<Team> {} bytes", t.n, size_of::<Id<Team>>());
}

// Example 2: typestate: methods that only exist in certain states.
struct Open;
struct Closed;

struct Door<State> {
    name: String,
    _state: PhantomData<State>,
}

impl Door<Closed> {
    fn new(name: &str) -> Self {
        Door { name: name.into(), _state: PhantomData }
    }

    fn open(self) -> Door<Open> {
        println!("  {} opens", self.name);
        Door { name: self.name, _state: PhantomData }
    }
}

impl Door<Open> {
    fn walk_through(&self) {
        println!("  walking through {}", self.name);
    }

    fn close(self) -> Door<Closed> {
        println!("  {} closes", self.name);
        Door { name: self.name, _state: PhantomData }
    }
}

fn demo_typestate() {
    println!("-- typestate --");

    let d = Door::new("front");

    // d.walk_through(); // error[E0599]: no method named `walk_through` found for `Door<Closed>`
    let d = d.open();
    d.walk_through();

    let _closed = d.close();
}

// Example 3: units of measure with zero runtime cost.
#[derive(Debug, Clone, Copy)]
struct Meters;
#[derive(Debug, Clone, Copy)]
struct Seconds;

#[derive(Debug, Clone, Copy)]
struct Quantity<U> {
    v: f64,
    _u: PhantomData<U>,
}

impl<U> std::ops::Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, o: Self) -> Self {
        Quantity { v: self.v + o.v, _u: PhantomData }
    }
}

fn demo_units() {
    println!("-- units --");

    let a: Quantity<Meters> = Quantity { v: 3.0, _u: PhantomData };
    let b: Quantity<Meters> = Quantity { v: 4.5, _u: PhantomData };
    let t: Quantity<Seconds> = Quantity { v: 2.0, _u: PhantomData };
    println!("a + b = {} m, t = {} s", (a + b).v, t.v);

    // a + t; // error[E0308]: mismatched types: expected `Quantity<Meters>`, found `Quantity<Seconds>`
}

fn name_len(s: &str) -> usize {
    s.len()
}

fn show<T: Display + ?Sized>(t: &T) -> String {
    format!("<{t}>")
}

fn show_sized<T: Display>(t: &T) -> String {
    format!("[{t}]")
}

// Example 4: Sized and ?Sized.
fn demo_sized() {
    println!("-- Sized / ?Sized --");

    let boxed: Box<str> = "hello".into();
    println!("{} {}", name_len(&boxed), size_of_val(&*boxed));
    println!("{} {}", show("a str"), show(&5)); // T = str is allowed because of ?Sized
    println!("{}", show_sized(&"a &str")); // T = &str, which is Sized
    // show_sized::<str>("x"); // error[E0277]: the size for values of type `str` cannot be known
    let slice: &[i32] = &[1, 2, 3];
    let d: &dyn Debug = &slice;
    println!("unsized values behind pointers: {slice:?} {d:?}");
}

fn describe(value: &dyn Any) -> String {
    if let Some(n) = value.downcast_ref::<i32>() {
        format!("i32 {n}")
    } else if let Some(s) = value.downcast_ref::<String>() {
        format!("String {s:?}")
    } else if value.is::<f64>() {
        "some f64".into()
    } else {
        "unknown".into()
    }
}

// Example 5: Any and downcasting.
fn demo_any() {
    println!("-- Any --");

    println!(
        "{} | {} | {} | {}",
        describe(&7i32),
        describe(&String::from("hi")),
        describe(&1.5f64),
        describe(&'c')
    );

    let items: Vec<Box<dyn Any>> =
        vec![Box::new(1i32), Box::new("str slice"), Box::new(2i32)];
    let ints: i32 = items.iter().filter_map(|b| b.downcast_ref::<i32>()).sum();
    println!("sum of the i32s {ints}");

    let b: Box<dyn Any> = Box::new(String::from("owned"));
    match b.downcast::<String>() {
        Ok(s) => println!("downcast Box to String {s}"),
        Err(_) => println!("not a String"),
    }
}

// Example 6: a type map keyed by TypeId.
struct Resources {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    fn insert<T: Any>(&mut self, v: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(v));
    }

    fn get<T: Any>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())?.downcast_ref()
    }
}

#[derive(Debug)]
struct Gravity(f64);

fn demo_type_map() {
    println!("-- type map --");

    let mut r = Resources { map: HashMap::new() };
    r.insert(Gravity(9.81));
    r.insert(String::from("level-1"));
    println!("{:?} {:?} {:?}", r.get::<Gravity>(), r.get::<String>(), r.get::<u8>());
    println!("g = {}", r.get::<Gravity>().map_or(0.0, |g| g.0));
}

// Example 7: type_name and TypeId.
fn demo_type_name() {
    println!("-- type_name / TypeId --");

    println!("{}", type_name::<Vec<i32>>());
    println!("{}", type_name::<Option<&str>>());

    let closure = |x: i32| x + 1;
    println!("of_val {} / {}", type_name_of_val(&1.0f32), type_name_of_val(&closure(1)));
    println!("TypeId equal for same type: {}", TypeId::of::<u8>() == TypeId::of::<u8>());
    println!("TypeId differs: {}", TypeId::of::<u8>() != TypeId::of::<i8>());
}

fn main() {
    demo_typed_ids();
    demo_typestate();
    demo_units();
    demo_sized();
    demo_any();
    demo_type_map();
    demo_type_name();
}
