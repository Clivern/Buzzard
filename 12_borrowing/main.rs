// 12. References and borrowing
//
// &T is a shared reference: you can have many at once, and none of them can change the
// value. &mut T is a unique reference: while it is alive, nothing else may read or write
// that value. That rule, one writer or many readers but never both, is checked at
// compile time and is why safe Rust has no data races.
//
// A borrow lasts until its last use, not until the end of the block (non-lexical
// lifetimes). References never dangle: a function cannot return a reference to its own
// local, because the local is dropped first.
//
// Run: cargo run --bin 12_borrowing

fn len(s: &String) -> usize {
    s.len()
}

fn push_bang(s: &mut String) {
    s.push('!');
}

// Example 1: lending a value instead of moving it.
fn demo_shared_refs() {
    println!("-- shared refs --");

    let s = String::from("rust");
    let n = len(&s);
    println!("{s} len {n}");

    let r1 = &s;
    let r2 = &s;
    let r3 = r1; // copying a shared reference is fine
    println!("{r1} {r2} {r3}");

    // Field access and method calls auto-dereference; * does it explicitly.
    let x = 5;
    let rx = &x;
    println!("*rx + 1 = {} same value? {}", *rx + 1, *rx == x);
}

// Example 2: mutable references.
fn demo_mut_refs() {
    println!("-- mut refs --");

    let mut s = String::from("hi");
    push_bang(&mut s);
    println!("{s}");

    let mut n = 10;
    let r = &mut n;
    *r += 5;
    println!("n {n}");

    // let fixed = String::new();
    // push_bang(&mut fixed);
    // error[E0596]: cannot borrow `fixed` as mutable, as it is not declared as mutable
}

// Example 3: the borrow rules, and NLL ending a borrow at its last use.
fn demo_borrow_rules() {
    println!("-- borrow rules --");

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    // r1 and r2 are not used again, so a mutable borrow is now allowed.
    let r3 = &mut s;
    r3.push_str(" rust");
    println!("{r3}");

    // let a = &mut s;
    // let b = &mut s;
    // println!("{a} {b}");
    // error[E0499]: cannot borrow `s` as mutable more than once at a time

    // let first = &s;
    // s.push('!');
    // println!("{first}");
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
}

// Example 4: the classic Vec trap, and how to restructure it.
fn demo_vec_trap() {
    println!("-- vec trap --");

    let mut v = vec![1, 2, 3];

    // let first = &v[0];
    // v.push(4);   // may reallocate, which would leave `first` dangling
    // println!("{first}");
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

    // Copy the value out first, then mutate.
    let first = v[0];
    v.push(4);
    println!("first {first} v {v:?}");

    // Or finish with the borrow before mutating.
    let total: i32 = v.iter().sum();
    v.push(total);
    println!("v {v:?}");
}

// Example 5: borrowing different parts at the same time.
fn demo_split_borrows() {
    println!("-- split borrows --");

    struct Account {
        name: String,
        balance: i64,
    }

    let mut acct = Account { name: String::from("ada"), balance: 10 };

    // Distinct fields can be borrowed independently.
    let name = &acct.name;
    let bal = &mut acct.balance;
    *bal += 5;
    println!("{name} {bal}");

    // Distinct halves of a slice need split_at_mut to prove they do not overlap.
    let mut nums = [1, 2, 3, 4];
    let (left, right) = nums.split_at_mut(2);
    left[0] += right[1];
    println!("{nums:?}");
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// fn dangle() -> &String {
//     let s = String::from("gone");
//     &s
// }
// error[E0106]: missing lifetime specifier
// (and with 'static added: error[E0515]: cannot return reference to local variable `s`)

fn no_dangle() -> String {
    String::from("owned")
}

// Example 6: no dangling references.
fn demo_no_dangling() {
    println!("-- no dangling --");

    let a = String::from("long");
    let b = String::from("tiny");
    println!("{}", longest(&a, &b));
    println!("{}", no_dangle());

    // let r;
    // {
    //     let short = String::from("short");
    //     r = &short;
    // }
    // println!("{r}");
    // error[E0597]: `short` does not live long enough
}

// Example 7: references to references, and comparing them.
fn demo_ref_of_ref() {
    println!("-- ref of ref --");

    let n = 7;
    let r = &n;
    let rr = &r;
    println!("{} {} {}", n, *r, **rr);

    // == compares the values behind references, not addresses.
    let m = 7;
    println!("values equal? {} same address? {}", &n == &m, std::ptr::eq(&n, &m));
}

fn main() {
    demo_shared_refs();
    demo_mut_refs();
    demo_borrow_rules();
    demo_vec_trap();
    demo_split_borrows();
    demo_no_dangling();
    demo_ref_of_ref();
}
