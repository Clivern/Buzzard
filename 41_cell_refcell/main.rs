// 41. Cell, RefCell, and interior mutability
//
// Normally you need &mut T to change a T. Interior mutability types let you change data
// through a shared &T, by moving the "only one writer" check somewhere else:
//
// - Cell<T>: get copies the value out, set replaces it. No references to the inside are
//   ever handed out, so no checks are needed. Best for small Copy values.
// - RefCell<T>: borrow() returns a Ref (like &T), borrow_mut() a RefMut (like &mut T).
//   The borrow rules are checked at runtime; breaking them panics.
// - Mutex, RwLock, and atomics (lesson 44) are the thread-safe versions.
//
// Cell and RefCell are not Sync, so they stay on one thread. Rc<RefCell<T>> is the
// classic single-threaded "shared and mutable" combination.
//
// Run: cargo run --bin 41_cell_refcell

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct Flag {
    hits: Cell<u32>,
}

impl Flag {
    // &self, yet it mutates.
    fn hit(&self) {
        self.hits.set(self.hits.get() + 1);
    }
}

// Example 1: Cell.
fn demo_cell() {
    println!("-- Cell --");

    let f = Flag { hits: Cell::new(0) };
    f.hit();
    f.hit();
    println!("hits {}", f.hits.get());

    let c = Cell::new(5);
    let old = c.replace(10);
    println!("replace old {old} new {}", c.get());

    let taken = Cell::new(String::from("text")).take(); // take needs Default
    println!("take {taken:?}");

    let a = Cell::new(1);
    let b = Cell::new(2);
    a.swap(&b);
    println!("swap {} {}", a.get(), b.get());
}

// Example 2: RefCell borrows.
fn demo_refcell() {
    println!("-- RefCell --");

    let v = RefCell::new(vec![1]);
    v.borrow_mut().push(2);
    {
        let r1 = v.borrow();
        let r2 = v.borrow(); // many readers is fine
        println!("{r1:?} {r2:?} len {}", r1.len());
    }

    {
        let mut w = v.borrow_mut();
        w.push(3);
    } // the RefMut is dropped here, ending the mutable borrow
    println!("{:?}", v.borrow());

    let inner: Vec<i32> = v.into_inner();
    println!("into_inner {inner:?}");
}

// Example 3: the runtime check, and try_borrow.
fn demo_runtime_check() {
    println!("-- runtime borrow check --");

    let cell = RefCell::new(0);
    let reader = cell.borrow();
    println!("try_borrow_mut while reading: {:?}", cell.try_borrow_mut().is_err());
    drop(reader);
    println!("after drop: {:?}", cell.try_borrow_mut().is_ok());

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let r = std::panic::catch_unwind(|| {
        let c = RefCell::new(1);
        let _a = c.borrow_mut();
        let _b = c.borrow_mut(); // panics: already mutably borrowed
    });

    std::panic::set_hook(hook);
    println!("double borrow_mut panicked: {}", r.is_err());
}

// Example 4: a cache behind &self.
struct Fib {
    memo: RefCell<HashMap<u64, u64>>,
    calls: Cell<u32>,
}

impl Fib {
    fn get(&self, n: u64) -> u64 {
        self.calls.set(self.calls.get() + 1);
        if n < 2 {
            return n;
        }

        if let Some(&v) = self.memo.borrow().get(&n) {
            return v;
        }

        // The shared borrow above has ended, so this mutable one is fine.
        let v = self.get(n - 1) + self.get(n - 2);
        self.memo.borrow_mut().insert(n, v);
        v
    }
}

fn demo_cache() {
    println!("-- memo cache --");

    let f = Fib { memo: RefCell::new(HashMap::new()), calls: Cell::new(0) };
    println!("fib(50) = {} with {} calls", f.get(50), f.calls.get());
}

// Example 5: Rc<RefCell<T>> for shared mutable state.
#[derive(Debug)]
struct Account {
    balance: i64,
}

fn demo_rc_refcell() {
    println!("-- Rc<RefCell<T>> --");

    let shared = Rc::new(RefCell::new(Account { balance: 100 }));
    let alice = Rc::clone(&shared);
    let bob = Rc::clone(&shared);
    alice.borrow_mut().balance -= 30;
    bob.borrow_mut().balance += 5;
    println!("{:?} owners {}", shared.borrow(), Rc::strong_count(&shared));
}

// Example 6: an observer list.
struct Button {
    listeners: RefCell<Vec<Box<dyn Fn(&str)>>>,
}

impl Button {
    fn on_click(&self, f: impl Fn(&str) + 'static) {
        self.listeners.borrow_mut().push(Box::new(f));
    }

    fn click(&self) {
        for l in self.listeners.borrow().iter() {
            l("clicked");
        }
    }
}

fn demo_observers() {
    println!("-- observers --");

    let log = Rc::new(RefCell::new(Vec::new()));
    let b = Button { listeners: RefCell::new(Vec::new()) };
    let log1 = Rc::clone(&log);
    b.on_click(move |e| log1.borrow_mut().push(format!("A got {e}")));

    let log2 = Rc::clone(&log);
    b.on_click(move |e| log2.borrow_mut().push(format!("B got {e}")));
    b.click();
    println!("{:?}", log.borrow());
}

// Example 7: Cell and RefCell are single-threaded.
fn demo_not_sync() {
    println!("-- not Sync --");

    // let c = std::sync::Arc::new(Cell::new(1));
    // std::thread::spawn(move || c.set(2));
    // error[E0277]: `Cell<i32>` cannot be shared between threads safely
    println!("Cell size == inner size: {} {}", size_of::<Cell<u64>>(), size_of::<u64>());
    println!("RefCell adds a borrow flag: {} bytes", size_of::<RefCell<u64>>());
}

fn main() {
    demo_cell();
    demo_refcell();
    demo_runtime_check();
    demo_cache();
    demo_rc_refcell();
    demo_observers();
    demo_not_sync();
}
