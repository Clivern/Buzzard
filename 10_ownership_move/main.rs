// 10. Ownership and moves
//
// Each value has exactly one owner. When the owner goes out of scope, the value is
// dropped and its resources are freed. Assigning a String, or passing it to a function,
// moves it: the old name can no longer be used. That rule is what prevents a double
// free without a garbage collector.
//
// Types that implement Copy, such as i32 and bool, are duplicated instead of moved
// (lesson 11). For everything else, return the value if the caller still needs it, or
// lend it with a reference (lesson 12).
//
// Moving one field out of a struct is a partial move: the other fields stay usable,
// but the struct as a whole does not.
//
// Run: cargo run --bin 10_ownership_move

// Example 1: assignment moves heap data; Copy types are duplicated.
fn demo_assignment() {
    println!("-- assignment --");

    let a = String::from("hello");
    let b = a;

    // println!("{a}");
    // error[E0382]: borrow of moved value: `a`
    println!("b owns it now: {b}");

    let n = 3;
    let m = n;
    println!("ints are Copy: {n} {m}");

    // A move copies only the (pointer, length, capacity) header, not the heap bytes.
    let v = vec![1, 2, 3];
    let p = v.as_ptr();
    let w = v;
    println!("same heap buffer after move? {}", std::ptr::eq(p, w.as_ptr()));
}

fn take(s: String) {
    println!("took {s}");
}

fn give_back(s: String) -> String {
    s
}

fn add_suffix(mut s: String) -> String {
    s.push_str("_v2");
    s
}

// Example 2: moving into and out of functions.
fn demo_functions() {
    println!("-- functions --");

    let s = String::from("hi");
    take(s);

    // take(s); // error[E0382]: use of moved value: `s`

    let t = String::from("there");
    let t = give_back(t);
    println!("got back {t}");

    let name = add_suffix(String::from("file"));
    println!("transformed {name}");
}

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn consume(n: Noisy) {
    println!("  consume got {}", n.0);
}

// Example 3: who drops what, and when.
fn demo_drop_timing() {
    println!("-- drop timing --");

    let a = Noisy("a");
    let b = Noisy("b");
    consume(a); // a is dropped at the end of consume
    println!("  back in caller");

    let c = b; // moving does not drop
    println!("  moved b into c");

    let _ = &c;
    println!("  end of scope next");
}

// Example 4: moves in loops and conditionals.
fn demo_conditional_move() {
    println!("-- conditional move --");

    let names = vec![String::from("ann"), String::from("ben")];
    for name in names {
        // each iteration takes ownership of one String
        println!("owned {name}");
    }

    // println!("{names:?}"); // error[E0382]: borrow of moved value: `names`

    let s = String::from("maybe");
    let keep = true;
    if keep {
        let moved = s;
        println!("moved into branch: {moved}");
    }

    // s is considered moved here, even though only one branch moved it.
}

#[derive(Debug)]
struct Pair {
    name: String,
    tag: String,
    n: i32,
}

// Example 5: partial moves out of a struct.
fn demo_partial_move() {
    println!("-- partial move --");

    let p = Pair { name: String::from("ada"), tag: String::from("x"), n: 1 };
    let name = p.name;
    println!("{name} {} {}", p.tag, p.n);

    // println!("{p:?}");
    // error[E0382]: borrow of partially moved value: `p`

    // Destructuring moves several fields at once.
    let q = Pair { name: String::from("bob"), tag: String::from("y"), n: 2 };
    let Pair { name, tag, .. } = q;
    println!("{name} {tag} n still readable: {}", q.n);
}

// Example 6: moving out of a Vec or Option without breaking the owner.
fn demo_take_out() {
    println!("-- taking values out --");

    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];

    // let first = v[0]; // error[E0507]: cannot move out of index of `Vec<String>`
    let last = v.pop().unwrap();
    let first = v.remove(0);
    let swapped = std::mem::replace(&mut v[0], String::from("new"));
    println!("{first} {last} {swapped} left {v:?}");

    let mut slot = Some(String::from("token"));
    let token = slot.take();
    println!("{token:?} left {slot:?}");
}

fn main() {
    demo_assignment();
    demo_functions();
    demo_drop_timing();
    demo_conditional_move();
    demo_partial_move();
    demo_take_out();
}
