mod front_house;
mod sub;
use front_house as front_house_import;
use front_house::another_mod as another_mod_import;
use front_house::another_mod::another_pub_fun as another_pub_fun_import;
use std::collections::HashMap;

extern crate rand;

// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn primitive_data_types() {
    println!("
Primitive Data Types:
---------------------
- bool : The boolean type.
- char : A character type.
- i8 : The 8-bit signed integer type.
- i16 : The 16-bit signed integer type.
- i32 : The 32-bit signed integer type.
- i64 : The 64-bit signed integer type.
- isize : The pointer-sized signed integer type.
- u8 : The 8-bit unsigned integer type.
- u16 : The 16-bit unsigned integer type.
- u32 : The 32-bit unsigned integer type.
- u64 : The 64-bit unsigned integer type.
- usize : The pointer-sized unsigned integer type.
- f32 : The 32-bit floating point type.
- f64 : The 64-bit floating point type.
- array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
- slice : A dynamically-sized view into a contiguous sequence, [T].
- str : String slices.
- tuple : A finite heterogeneous sequence, (T, U, ..).

----> Ref: https://doc.rust-lang.org/book/ch03-02-data-types.html
----> Ref: https://doc.rust-lang.org/rust-by-example/index.html
    ");
}

fn rust_docs() {
    println!(
        "
Rust docs in browser:
--------------------
$ rustup doc --book
$ rustup doc --std
$ rustup doc --cargo
    "
    );
}

fn sqrt(x: f64) -> f64 {
    return x.sqrt();
}

// Functions return the last statement value by default.
fn mutiple_of_two(x: f64) -> f64 {
    x * 2.0
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// Read-only borrow.
fn by_ref(x: &f64) -> f64 {
    *x + 1.0
}

// Mutable borrow.
fn modifies(x: &mut f64) {
    *x += 1.0;
}

// we use slices
fn arr_sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn dump(arr: &[i32]) {
    // If type is defined with #[derive(Debug)], you may use debug print {:?} in the format string.
    println!("arr is {:?}", arr);
}

// Return a tuple
fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// Here, some_integer goes out of scope. Nothing special happens.
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = "Hello".to_string();
    some_string
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn borrow(a: &String) -> usize {
    a.len()
}

//fn change(a: &String) {
//    a.push_str(", world");
//}

fn change_mut(a: &mut String) {
    a.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);

fn dimensions(dim: (i32, i32)) -> i32 {
    dim.0 * dim.1
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn area(rt: &Rectangle) -> i32 {
    rt.width * rt.height
}

#[allow(dead_code)]
enum IpType {
    V4,
    V6,
}

struct IpAddress {
    address: String,
    kind: IpType,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Currency {
    Dollar,
    Euro,
}

fn get_sign(cur: Currency) -> String {
    match cur {
        Currency::Euro => "Euro".to_string(),
        Currency::Dollar => "Dollar".to_string(),
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn is_even(n: i32) -> Result<bool, String> {
    if n % 2 == 0 {
        return Ok(true);
    }

    return Err("Not an even".to_string());
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct PointGeneric<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointGeneric<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &U {
        &self.y
    }
}

pub trait Summary {
    fn summarize(&self) -> String;

    // Default Implementations
    fn summarize_default(&self) -> String {
        "@default implementations".to_string()
    }
}

pub struct Article {
    pub headline: String,
}

pub struct Tweet {
    pub username: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

// Traits as Parameters
// Traits is just interfaces with default implementation ability
fn summarize_001(item: &impl Summary) -> String {
    item.summarize()
}

// Returning Types that Implement Traits
fn new_article() -> impl Summary {
    Article {
        headline: "Hello World".to_string(),
    }
}

// Lifetime Annotations in Function Signatures
// The help text reveals that the return type needs a generic lifetime parameter on it because
// Rust can’t tell whether the reference being returned refers to x or y
fn longest_01<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_02(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

const HUBBLE_CONSTANT: f64 = 69.8; // July 2019 measurement, units: (km/sec) / megaparsec

fn main() {
    //////// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
    //////// https://doc.rust-lang.org/std/index.html
    println!("----------------- Basics -----------------");

    primitive_data_types();
    rust_docs();

    println!("Hello, world!");

    let x = 32;
    println!("{}", x);

    let name = "Clivern";
    println!("{name}", name = name);

    for i in 0..5 {
        println!("i = {}", i); // 0, 1, 2, 3, 4
    }

    for i in 0..5 {
        if i % 2 == 0 {
            println!("Even: {}", i);
        } else {
            println!("Odd: {}", i);
        }
    }

    for i in 0..5 {
        println!("{} {}", if i % 2 == 0 { "Even:" } else { "Odd:" }, i);
    }

    //////// let variables by default can only be assigned a value when declared.
    //////// Adding the word mut (please make this variable mutable)
    let mut y = 0;

    for i in 0..5 {
        y += i;
    }
    assert_eq!(y, 0 + 1 + 2 + 3 + 4);

    println!("{}", sqrt(2.0));
    println!("{}", sqrt(2.1));
    println!("{}", mutiple_of_two(2.1));

    println!("Abs 2.0: {}", abs(2.0));
    println!("Abs -2.0: {}", abs(-2.0));

    let mut h = 20.0;
    println!("{}", by_ref(&h)); // 21
    println!("{}", h); // 20
    modifies(&mut h);
    println!("{}", h); // 21
    println!("{}", h.abs());

    //////// Constants
    println!("{}", std::env::consts::OS);
    println!("{}", std::env::consts::FAMILY);
    println!("{}", std::env::consts::ARCH);

    // Basic mathematical constants -> https://doc.rust-lang.org/std/f64/consts/index.html
    println!("PI -> {}", std::f64::consts::PI);

    // Program-wide constants
    assert_eq!(HUBBLE_CONSTANT, 69.8);

    //////// Arrays
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    //arr[0] = 11; -> will fail
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }

    println!("length {}", arr.len());
    println!("sum ~> {}", arr_sum(&arr));

    let mut arr1 = [10, 20, 30, 40];
    arr1[0] = 11;
    println!("sum ~> {}", arr_sum(&arr1));

    //////// Slicing and Dicing
    //////// you can do a debug print with {:?}
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);

    let ints_arr = [1, 2, 3, 4, 5];
    let slice1 = &ints_arr[0..2];
    let slice2 = &ints_arr[1..]; // open range!
    assert_eq!(slice1, [1, 2]);
    assert_eq!(slice2, [2, 3, 4, 5]);

    let first = slice1.get(0);
    let last = slice1.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);

    //////// Option
    let msg: Option<&str> = Some("howdy");
    let msg_num: Option<u32> = Some(2);
    assert_eq!(false, msg_num.is_none());
    assert_eq!(false, msg.is_none());

    //////// Vector: These are re-sizeable arrays
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let v_first = v[0]; // will panic if out-of-range
    let v_maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", v_first);
    println!("maybe_first is {:?}", v_maybe_first);
    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);

    // iterators
    let it_arr = [10, 20, 30];
    for i in it_arr.iter() {
        assert_eq!(it_arr.contains(i), true);
    }

    let it_arr02 = 0..5;
    for i in it_arr02 {
        assert_eq!([0, 1, 2, 3, 4].contains(&i), true);
    }

    let sum01: i32 = (0..5).sum();
    assert_eq!(0 + 1 + 2 + 3 + 4, sum01);

    let sum02: i64 = [10, 20, 30].iter().sum();
    assert_eq!(10 + 20 + 30, sum02);

    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);
    v2.extend(0..2);

    println!("{:?}", v1);
    println!("{:#?}", v1);
    assert_eq!(v1, [10, 20, 30]);
    assert_eq!(v2, [10, 20, 30, 0, 1]);

    let mut v3 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v3.sort();
    assert_eq!(v3, [1, 1, 2, 2, 5, 10, 11, 40]);

    //////// Matching
    let n = 1;
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };

    assert_eq!(text, "one".to_string());

    let size = match n {
        0..=3 => "small",
        4..=6 => "medium",
        _ => "large",
    };

    assert_eq!(size, "small".to_string());

    let mut stext01 = String::new();
    stext01.push('H');
    let stext02 = " He llo ";

    assert_eq!(stext01, "H".to_string());
    assert_eq!(stext02, " He llo ".to_string());

    let stripped: String = stext02.chars().filter(|ch| !ch.is_whitespace()).collect();

    assert_eq!(stripped, "Hello".to_string());

    for arg in std::env::args() {
        println!("ARG -> '{}'", arg);
    }

    let s1 = "hello dolly".to_string();
    let _s2 = s1;
    println!("s1 {}", _s2);

    //////// Tuple
    let tuple = ("hello", 5, "world");
    let (add, mul) = add_mul(3.0, 2.0);  // destructuring tuple assignment using let

    assert_eq!(tuple.0, "hello".to_string());
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, "world".to_string());
    assert_eq!(add, 5.0);
    assert_eq!(mul, 6.0);

    for t in ["zero", "one", "two"].iter().enumerate() {
        match t.0 {
            0 => assert_eq!(t.1.to_string(), "zero"),
            1 => assert_eq!(t.1.to_string(), "one"),
            _ => assert_eq!(t.1.to_string(), "two"),
        }
    }

    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];
    for p in names.iter().zip(nums.iter()) {
        match p.1 {
            10 => assert_eq!(p.0.to_string(), "ten"),
            100 => assert_eq!(p.0.to_string(), "hundred"),
            _ => assert_eq!(p.0.to_string(), "thousand"),
        }
    }

    let mut p = Person::new("John", "Smith");
    assert_eq!(p.first_name, "John".to_string());
    assert_eq!(p.last_name, "Smith".to_string());
    p.set_first_name("Joe");
    p.set_last_name("Doe");
    assert_eq!(p.to_tuple(), ("Joe".to_string(), "Doe".to_string()));

    let x = format!("{}, {}!", "Hello", "world");
    assert_eq!(x, "Hello, world!");

    // Shadowing
    // you can declare a new variable with the same name as a previous variable,
    // and the new variable shadows the previous variable.

    let sh_var = 5;
    let sh_var = sh_var + 1;
    let sh_var = sh_var * 2;
    assert_eq!(sh_var, 12);

    // with shadowing we can change the var type
    let sh_str = "hello";
    assert_eq!(sh_str, "hello");

    let sh_str = sh_str.len();
    assert_eq!(sh_str, 5);

    let heart_eyed_cat = "😻";
    assert_eq!(heart_eyed_cat, "😻");

    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.
    let mut tup = (64, 64.0, "Hello");
    assert_eq!(tup.0, 64);
    assert_eq!(tup.1, 64.0);
    assert_eq!(tup.2, "Hello");
    tup.0 = 77;
    assert_eq!(tup.0, 77);

    let tupl: (i32, f64, String) = (50, 6.4, "Hello".to_string());
    assert_eq!(tupl.0, 50);
    assert_eq!(tupl.1, 6.4);
    assert_eq!(tupl.2, "Hello".to_string());

    let a_aa: [i64; 2] = [1, 2];
    assert_eq!(a_aa[0], 1);
    assert_eq!(a_aa[1], 2);

    let z = 4;
    let c = {
        let z = 6;
        z * 3
    };
    assert_eq!(z, 4);
    assert_eq!(c, 18);

    let number = 3;
    assert_eq!(true, number < 5);
    assert_eq!(true, number == 3);

    let text = "Hello";
    assert_eq!(text, "Hello");

    let t = 20;

    if t > 0 && t < 10 {
        println!("t is more than 0 && less than 10");
    } else if t < 20 && t > 10 {
        println!("t is more than 10 && less than 20");
    } else {
        println!("t is more than 20");
    }

    let ol1 = if t >= 20 { 3 } else { 4 };
    let ol2 = {
        if t > 20 {
            3
        } else {
            4
        }
    };

    assert_eq!(ol1, 3);
    assert_eq!(ol2, 4);

    let mut inc = 0;

    loop {
        if inc == 3 {
            break;
        }
        assert_eq!(true, [0, 1, 2].contains(&inc));
        inc += 1;
    }

    assert_eq!(3, inc);

    let mut inc01 = 0;
    let result = loop {
        inc01 += 1;
        if inc01 == 3 {
            break inc01;
        }
    };

    assert_eq!(3, result);

    let mut yl = 0;
    while yl < 10 {
        yl += 1;
    }
    assert_eq!(10, yl);

    let mut text09 = "Hello".to_string();
    text09 += " World";
    text09.push_str("!");
    assert_eq!(text09, "Hello World!");

    {
        let scope = "Hello".to_string();
        assert_eq!(scope, "Hello");
    }
    // scope gone, In rust the memory is automatically returned once the variable that owns it goes out of scope
    // When a variable goes out of scope, Rust calls a special function for us.
    // This function is called drop, and it’s where the author of String can put the code to return the memory.
    // Rust calls drop automatically at the closing curly bracket.

    let x_001 = 5;
    let y_001 = x_001; //  make a copy of the value in x_001 and bind it to y_001
                       // now we have both y_001 and x_001
    assert_eq!(x_001, 5);
    assert_eq!(y_001, 5);

    let s_001 = "Hello".to_string();
    let s_002 = s_001;
    // Rust won't copy the value here too because it could be very expensive in terms of runtime performance if the data on the heap were large.
    // also it won't alllow a reference to same value because when s_001 and s_002 go out of scope,
    // they will both try to free the same memory. This is known as a double free error
    //  Instead rust conside s_001 not valid anymore and doesn’t need to free anything when s_001 goes out of scope.

    //assert_eq!(s_001, "Hello"); // will throw error since value
    assert_eq!(s_002, "Hello");

    // If we do want to deeply copy the heap data of the String
    let s_003 = "Hello".to_string();
    let s_004 = s_003.clone();

    assert_eq!(s_003, "Hello");
    assert_eq!(s_004, "Hello");

    let x_001 = String::from("hello");

    takes_ownership(x_001.clone());

    //println!("{:?}", x_001); -> error value borrowed here after move into takes_ownership

    let y_001 = "Hello".to_string();
    takes_ownership(y_001.clone()); // takes a copy so y_001 still on the scope
    assert_eq!(y_001, "Hello");

    let x_008 = 5;
    makes_copy(x_008);
    assert_eq!(x_008, 5);

    let z_0001 = gives_ownership();
    assert_eq!(z_0001, "Hello");

    let z_0002 = "Hello".to_string();
    assert_eq!(z_0002, "Hello");

    let z_0003 = takes_and_gives_back(z_0002);
    assert_eq!(z_0003, "Hello");
    // assert_eq!(z_0002, "Hello"); -> fails since z_0002 not valid anymore

    // The &k_001 syntax lets us create a reference that refers to the value of k_001 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
    let k_001 = "Hello".to_string();
    let k_002 = borrow(&k_001);
    assert_eq!(k_001, "Hello");
    assert_eq!(k_002, 5);

    // change(&k_001); -> error because just as variables are immutable by default,
    // so are references. We’re not allowed to modify something we have a reference to.

    // First, we had to change k_003 to be mut.
    // Then we had to create a mutable reference with &mut k_003 and accept a mutable reference with some_string: &mut String.
    let mut k_003 = "Hello".to_string();
    change_mut(&mut k_003);
    assert_eq!(k_003, "Hello, world");

    // The Slice Type (&str)
    // A string slice is a reference to part of a String, and it looks like this:
    let str_total = "Hello".to_string();
    let slice_001 = &str_total[0..2];
    assert_eq!(slice_001, "He");

    let word_001 = "Hello World!".to_string();
    let word_002 = "HelloWorld!".to_string();
    assert_eq!(first_word(&word_001), "Hello");
    assert_eq!(first_word(&word_002), "HelloWorld!");

    // String Literals Are Slices
    let str_lit_001 = "HelloWorld!";
    assert_eq!(first_word(&word_002), str_lit_001);

    // there’s a more general slice type, too. Consider this array:
    let arr_0001 = [1, 2, 3, 4, 5];
    let slice_0001 = &arr_0001[1..3]; // This slice has the type &[i32].
    assert_eq!(slice_0001, [2, 3]);

    let user_001 = User {
        username: "admin".to_string(),
        email: "hello@clivern.com".to_string(),
        sign_in_count: 1,
        active: true,
    };

    assert_eq!(user_001.username, "admin");
    assert_eq!(user_001.email, "hello@clivern.com");
    assert_eq!(user_001.sign_in_count, 1);
    assert_eq!(user_001.active, true);

    let point_001 = Point(1, 2, 3);
    assert_eq!(point_001.0, 1);
    assert_eq!(point_001.1, 2);
    assert_eq!(point_001.2, 3);

    assert_eq!(16, dimensions((4, 4)));

    let rt_001 = Rectangle {
        width: 4,
        height: 4,
    };

    assert_eq!(16, area(&rt_001));
    assert_eq!(16, rt_001.area());

    let ip_0001 = IpAddress {
        address: "127.0.0.1".to_string(),
        kind: IpType::V4,
    };

    assert_eq!(ip_0001.address, "127.0.0.1");

    let type_001 = match ip_0001.kind {
        IpType::V4 => "V4",
        IpType::V6 => "V6",
    };
    assert_eq!(type_001, "V4");

    let _ip_addr_v4 = IpAddr::V4(127, 0, 0, 1);
    let _ip_addr_v6 = IpAddr::V6("::1".to_string());

    let some_num_001 = Some(5); // -> will be of type Option<i8>
    let some_str_001 = Some("Hello World"); // -> will be of type Option<str>
    let absent_001: Option<i32> = None;
    let absent_002 = None; // will be of type Option<T>
    assert_eq!(absent_002, absent_001);

    assert_eq!(some_num_001.unwrap(), 5);
    assert_eq!(some_str_001.unwrap(), "Hello World");

    // Matching
    assert_eq!(get_sign(Currency::Euro), "Euro");
    assert_eq!(get_sign(Currency::Dollar), "Dollar");

    let cur_001 = Currency::Euro;
    match cur_001 {
        Currency::Euro => {
            println!("{:?}", cur_001); // -> Euro
        }
        Currency::Dollar => {
            println!("{:?}", cur_001);
        }
    }

    // Matching with Option<T>
    let five_001 = Some(5);
    let six_001 = match five_001 {
        None => None,
        Some(i) => Some(i + 1),
    };
    assert_eq!(Some(6), six_001);

    // if let is syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    if let Some(6) = six_001 {
        assert_eq!(true, true);
    } else {
        assert_eq!(true, false);
    }

    // Modules allow disambiguation between items that have the same name.
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the mode specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line

    // Ref -> https://medium.com/@tak2siva/rust-modules-explained-96809931bbbf
    assert_eq!(
        front_house::another_mod::another_pub_fun(),
        "Hello".to_string()
    );

    assert_eq!(
        front_house_import::another_mod::another_pub_fun(),
        "Hello".to_string()
    );

    assert_eq!(another_mod_import::another_pub_fun(), "Hello".to_string());

    assert_eq!(another_pub_fun_import(), "Hello".to_string());
    assert_eq!(
        sub::back_yard::another_mod::another_pub_fun(),
        "Hello".to_string()
    );

    println!("{:?}", rand::random::<f64>());

    let mut vec_001: Vec<i32> = Vec::new();
    vec_001.push(1);
    vec_001.push(2);
    vec_001.push(3);
    vec_001.push(4);
    vec_001.push(5);

    let vec_002 = vec![1, 2, 3, 4, 5];

    let mut vec_003 = vec![1, 2, 3, 4, 5];
    vec_003.push(6);
    assert_eq!(vec_003, vec![1, 2, 3, 4, 5, 6]);

    // vec_002.push(2); -> will fail

    for i in vec_002 {
        assert_eq!(true, [1, 2, 3, 4, 5].contains(&i));
    }

    for i in &mut vec_001 {
        *i += 1; // will update value inside the vec
    }

    assert_eq!(vec_001, vec![2, 3, 4, 5, 6]);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text("blue".to_string()),
        SpreadsheetCell::Float(10.12),
    ];
    assert_eq!(row.len(), 3);

    // ///////
    // for more vec -> https://doc.rust-lang.org/std/vec/struct.Vec.html
    // ///////

    let mut text_0001 = String::from("Hello");
    text_0001.push_str(" World");
    text_0001.push('.'); // -> only to append one char

    for c in text_0001.chars() {
        assert_eq!(
            true,
            ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '.'].contains(&c)
        );
    }

    // ///////////
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    // ///////////

    let mut scores_001 = HashMap::new();

    scores_001.insert("blue".to_string(), 10);
    scores_001.insert("yellow".to_string(), 50);

    for (key, value) in &scores_001 {
        assert_eq!(scores_001.get(key).unwrap(), value);
    }

    let text_003 = "hello world wonderful world";
    let mut word_map_001 = HashMap::new();

    for word in text_003.split_whitespace() {
        let count = word_map_001.entry(word).or_insert(0);
        *count += 1;
    }

    assert_eq!(*word_map_001.get("hello").unwrap(), 1);
    assert_eq!(*word_map_001.get("wonderful").unwrap(), 1);
    assert_eq!(*word_map_001.get("world").unwrap(), 2);
    // println!("{:?}", word_map_001); // {"world": 2, "hello": 1, "wonderful": 1}

    // Unrecoverable Errors
    // export RUST_BACKTRACE=1 will show stacktrace
    // panic!("crash and burn");

    // ///////////////
    // Error Handling
    // ///////////////
    let i07 = is_even(7);

    match i07 {
        Ok(d) => {
            assert_eq!(i07, Ok(true));
            assert_eq!(d, true);
            println!("An even {:?}", i07);
        }
        Err(err) => {
            assert_eq!("Not an even".to_string(), err);
            println!("{:?}", err);
        }
    }

    let i08 = is_even(8);

    match i08 {
        Ok(d) => {
            assert_eq!(i08, Ok(true));
            assert_eq!(d, true);
            println!("An even {:?}", i08);
        }
        Err(err) => {
            assert_eq!("Not an even".to_string(), err);
            println!("{:?}", err);
        }
    }

    // The unwrap() function returns the actual result an operation succeeds.
    // It returns a panic with a default error message if an operation fails.
    // This function is a shorthand for match statement.
    let i_10 = is_even(10);

    assert_eq!(i_10.unwrap(), true);

    let _i_09 = is_even(9);
    // i_09.unwrap(); -> will panic

    // let i_19 = is_even(19).expect("19 not even so panic"); // thread 'main' panicked at '19 not even so panic: "Not an even"', src/libcore/result.rs:1165:5

    let num_list_001 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let num_list_002 = vec![11, 21, 31, 41, 51, 61, 71, 81, 91, 101];

    assert_eq!(100, largest(&num_list_001));
    assert_eq!(101, largest(&num_list_002));

    let chars_list_001 = vec!['d', 'a', 'b', 'e', 'c'];
    assert_eq!('e', largest_char(&chars_list_001));

    let generic_point_001 = PointGeneric { x: 3, y: 4.0 };

    assert_eq!(generic_point_001.x, 3);
    assert_eq!(*generic_point_001.get_x(), 3);
    assert_eq!(generic_point_001.y, 4.0);
    assert_eq!(*generic_point_001.get_y(), 4.0);

    // Defining Shared Behavior
    let article_001 = Article {
        headline: "Hello World".to_string(),
    };
    let article_002 = new_article();
    let tweet_001 = Tweet {
        username: "admin".to_string(),
    };

    assert_eq!("Hello World".to_string(), article_001.summarize());
    assert_eq!("Hello World".to_string(), article_002.summarize());
    assert_eq!("admin".to_string(), tweet_001.summarize());
    assert_eq!("Hello World".to_string(), summarize_001(&article_001));
    assert_eq!("Hello World".to_string(), summarize_001(&article_002));
    assert_eq!("admin".to_string(), summarize_001(&tweet_001));

    assert_eq!(
        "@default implementations".to_string(),
        article_001.summarize_default()
    );
    assert_eq!(
        "@default implementations".to_string(),
        article_002.summarize_default()
    );
    assert_eq!(
        "@default implementations".to_string(),
        tweet_001.summarize_default()
    );

    let res_0001 = "Hello";
    let res_0002 = "World ";
    let res_0003 = longest_01(res_0001, res_0002);
    assert_eq!("World ", res_0003);

    let res_0003 = longest_02("Hello".to_string(), "World ".to_string());
    assert_eq!("World ".to_string(), res_0003);

    // Iterators
    let list_001 = vec![1, 2, 3, 4, 5];
    let total_001: i32 = list_001.iter().sum();
    assert_eq!(total_001, 15);
    let list_002: Vec<_> = list_001.iter().map(|x| x + 1).collect();

    println!("{:?}", list_002);
    assert_eq!(vec![2, 3, 4, 5, 6], list_002);
}
