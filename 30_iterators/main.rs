// 30. Iterators and IntoIterator
//
// An iterator produces a sequence of values, one per call to next(), until next()
// returns None. The Iterator trait needs only that one method; the dozens of adapters
// and consumers (map, filter, sum, collect...) are default methods built on it.
//
// A collection gives you three iterators. iter() yields &T and borrows. iter_mut()
// yields &mut T. into_iter() yields T and consumes the collection. A for loop calls
// into_iter() on whatever you give it, so for x in &v is iter(), for x in &mut v is
// iter_mut(), and for x in v is into_iter().
//
// Iterators are lazy: nothing happens until something pulls values. Implementing
// Iterator for your own type takes one method and unlocks the whole toolbox.
//
// Run: cargo run --bin 30_iterators

// Example 1: calling next() by hand.
fn demo_next() {
    println!("-- next --");

    let v = vec![1, 2, 3];
    let mut it = v.iter();
    println!("{:?} {:?} {:?} {:?}", it.next(), it.next(), it.next(), it.next());

    // A fused iterator keeps returning None.
    println!("again {:?}", it.next());
    println!("count {}", [1, 2, 3].iter().count());
}

// Example 2: iter, iter_mut, and into_iter.
fn demo_three_iters() {
    println!("-- iter / iter_mut / into_iter --");

    let mut v = vec![String::from("a"), String::from("b")];
    for s in v.iter() {
        print!("borrow {s} ");
    }

    println!();
    for s in v.iter_mut() {
        s.push('!');
    }

    println!("after iter_mut {v:?}");
    let owned: Vec<String> = v.into_iter().map(|s| s + "?").collect();
    println!("into_iter moved: {owned:?}");

    // println!("{v:?}"); // error[E0382]: borrow of moved value: `v`
}

// Example 3: what a for loop desugars to.
fn demo_for_desugar() {
    println!("-- for desugar --");

    let v = vec![10, 20];
    for n in &v {
        print!("{n} ");
    }

    println!();

    // The same loop, written out:
    let mut it = IntoIterator::into_iter(&v);
    while let Some(n) = it.next() {
        print!("{n} ");
    }

    println!();

    // Arrays iterate by value too.
    for s in ["x", "y"] {
        print!("{s} ");
    }

    println!();
}

// Example 4: laziness.
fn demo_lazy() {
    println!("-- laziness --");

    let it = [1, 2, 3].iter().map(|n| {
        println!("  mapping {n}");
        n * 2
    });

    println!("nothing mapped yet");
    let v: Vec<_> = it.collect();
    println!("collected {v:?}");

    // An unused adapter chain warns: iterators are lazy and do nothing unless consumed.
}

// Example 5: implementing Iterator.
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

struct Fib {
    a: u64,
    b: u64,
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let out = self.a;
        self.a = self.b;
        self.b = out.checked_add(self.b)?;
        Some(out)
    }
}

fn demo_custom_iterator() {
    println!("-- custom Iterator --");

    let v: Vec<_> = Countdown(5).collect();
    println!("countdown {v:?}");
    println!("sum {} max {:?}", Countdown(4).sum::<u32>(), Countdown(4).max());

    let fib: Vec<_> = Fib { a: 0, b: 1 }.take(10).collect();
    println!("fib {fib:?}");
    println!("fib terms before u64 overflow {}", Fib { a: 0, b: 1 }.count());
}

// Example 6: IntoIterator for your own collection, by value and by reference.
struct Team {
    members: Vec<String>,
}

impl IntoIterator for Team {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

impl<'a> IntoIterator for &'a Team {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.members.iter()
    }
}

fn demo_into_iterator() {
    println!("-- IntoIterator --");

    let team = Team { members: vec!["ann".into(), "ben".into()] };
    for m in &team {
        print!("{m} ");
    }

    println!();
    for m in team {
        print!("owned {m} ");
    }

    println!();
}

// Example 7: double-ended and exact-size iterators.
fn demo_double_ended() {
    println!("-- double ended --");

    let v = [1, 2, 3, 4, 5];
    let mut it = v.iter();
    println!("front {:?} back {:?} len left {}", it.next(), it.next_back(), it.len());
    println!("rev {:?}", v.iter().rev().collect::<Vec<_>>());
    println!("last {:?} nth(1) {:?}", v.iter().last(), v.iter().nth(1));
}

fn main() {
    demo_next();
    demo_three_iters();
    demo_for_desugar();
    demo_lazy();
    demo_custom_iterator();
    demo_into_iterator();
    demo_double_ended();
}
