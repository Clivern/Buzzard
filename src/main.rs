
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

fn by_ref(x: &f64) -> f64{
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

fn main() {
    // https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
    println!("----------------- Basics -----------------");

    primitive_data_types();
    rust_docs();

    println!("Hello, world!");

    let x = 32;
    println!("{}", x);

    let name = "Clivern";
    println!("{name}", name=name);

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
        println!("{} {}", if i % 2 == 0 {"Even:"} else {"Odd:"}, i);
    }

    // let variables by default can only be assigned a value when declared.
    // Adding the word mut (please make this variable mutable)
    let mut y = 0;

    for i in 0..5 {
        y += i;
    }

    println!("Y = {}", y);

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

    // Constants
    println!("{}", std::env::consts::OS);
    println!("{}", std::env::consts::FAMILY);
    println!("{}", std::env::consts::ARCH);

    // Arrays
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    //arr[0] = 11; -> will fail
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }

    println!("length {}", arr.len());
    println!("sum ~> {}", arr_sum(&arr));


    let mut arr1 = [10, 20, 30, 40];
    arr1[0] = 11;
    println!("sum ~> {}", arr_sum(&arr1));

    // Slicing and Dicing
    // you can do a debug print with {:?}
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
    let slice2 = &ints_arr[1..];  // open range!
    println!("ints_arr {:?}", ints_arr);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);

    let first = slice1.get(0);
    let last = slice1.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);

    // Option
    let msg: Option<&str> = Some("howdy");
    let msg_num: Option<u32> = Some(2);
    println!("{}", msg.is_none()); // false
    println!("{}", msg_num.is_none()); // false

    // Vector: These are re-sizeable arrays
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let v_first = v[0];  // will panic if out-of-range
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
        println!("{}", i);
    }

    let it_arr02 = 0..5;
    for i in it_arr02 {
        println!("{}", i);
    }

    let sum01: i32  = (0..5).sum();
    println!("sum was {}", sum01);

    let sum02: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum02);


    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);
    v2.extend(0..2);

    println!("{:?}", v1); // [10, 20, 30]
    println!("{:?}", v2); // [10, 20, 30, 0, 1]

    let mut v3 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v3.sort();
    println!("{:?}", v3); // [1, 1, 2, 2, 5, 10, 11, 40]

    // Matching
    let n = 1;
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };
    println!("{:?}", text); // one

    let size = match n {
        0..=3 => "small",
        4..=6 => "medium",
        _ => "large",
    };
    println!("{:?}", size); // small

    let mut stext01 = String::new();
    stext01.push('H');

    let stext02 = " He llo ";

    println!("{:?}", stext01); // H
    println!("{:?}", stext02); //  He llo

    let stripped: String = stext02.chars()
        .filter(|ch| !ch.is_whitespace()).collect();

    println!("{:?}", stripped); // Hello

    for arg in std::env::args() {
        println!("ARG -> '{}'", arg);
    }
}

fn primitive_data_types(){
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

fn rust_docs(){
    println!("
Rust docs in browser:
--------------------
$ rustup doc --std
    ");
}
