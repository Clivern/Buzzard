fn sqrt(x: f64) -> f64 {
    return x * x;
}

// retuns the last statement value by default
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

fn by_ref(x: &f64) -> f64 {
    *x + 1.0
}

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
    println!("arr is {:?}", arr);
}

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
    let (add, mul) = add_mul(3.0, 2.0);

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
    ");
}

fn rust_docs() {
    println!(
        "
Rust docs in browser:
--------------------
$ rustup doc --std
    "
    );
}
