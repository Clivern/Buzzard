// 46. Send, Sync, Condvar, and Barrier
//
// Send means a value can be moved to another thread. Sync means a &T can be shared
// between threads; T is Sync exactly when &T is Send. The compiler implements both
// automatically for types made of Send/Sync parts. Rc is neither, Arc<T> is both when T
// is, Cell and RefCell are Send but not Sync, and Mutex<T> makes a Send T Sync. These
// two marker traits are how safe Rust rules out data races at compile time.
//
// Condvar lets a thread sleep until another thread says something changed. You hold a
// MutexGuard, call wait (which releases the lock while sleeping), and get the guard back
// when woken. Always re-check the condition in a loop (or use wait_while): wakeups can
// be spurious.
//
// Barrier makes n threads wait until all n have arrived, then releases them together.
// It splits parallel work into phases.
//
// Run: cargo run --bin 46_sync_primitives

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn is_send<T: Send>() -> bool {
    true
}
fn is_sync<T: Sync>() -> bool {
    true
}

// Example 1: checking Send and Sync at compile time.
fn demo_send_sync() {
    println!("-- Send / Sync --");

    println!("i32 send {} sync {}", is_send::<i32>(), is_sync::<i32>());
    println!("String send {} sync {}", is_send::<String>(), is_sync::<String>());
    println!(
        "Arc<Mutex<Vec<u8>>> send {} sync {}",
        is_send::<Arc<Mutex<Vec<u8>>>>(),
        is_sync::<Arc<Mutex<Vec<u8>>>>()
    );
    println!("Cell<i32> send {}", is_send::<Cell<i32>>());
    println!("RefCell<i32> send {}", is_send::<RefCell<i32>>());

    // is_sync::<Cell<i32>>();  // error[E0277]: `Cell<i32>` cannot be shared between threads safely
    // is_send::<Rc<i32>>();    // error[E0277]: `Rc<i32>` cannot be sent between threads safely
    // is_sync::<Rc<i32>>();    // error[E0277]: `Rc<i32>` cannot be shared between threads safely
    let _ = Rc::new(()); // Rc is fine on one thread
}

// Example 2: a struct is Send/Sync only if its fields are.
struct Holder {
    _data: Vec<u8>,
}

struct NotSync {
    _flag: Cell<bool>,
}

fn demo_auto_traits() {
    println!("-- auto traits --");

    println!("Holder send {} sync {}", is_send::<Holder>(), is_sync::<Holder>());
    println!("NotSync send {}", is_send::<NotSync>());

    // is_sync::<NotSync>(); // error[E0277]: `Cell<bool>` cannot be shared between threads safely
    // `unsafe impl Sync for NotSync {}` would compile, but would be a lie that allows
    // data races. Only do that for types whose synchronization you implemented yourself.
}

// Example 3: Condvar signalling readiness.
fn demo_condvar() {
    println!("-- Condvar --");

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        let (lock, cv) = &*pair2;
        *lock.lock().unwrap() = true;
        cv.notify_one();
    });

    let (lock, cv) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cv.wait(ready).unwrap();
    }

    println!("ready");
}

// Example 4: a blocking queue built from Mutex + Condvar.
struct BlockingQueue<T> {
    items: Mutex<VecDeque<T>>,
    not_empty: Condvar,
}

impl<T> BlockingQueue<T> {
    fn new() -> Self {
        Self { items: Mutex::new(VecDeque::new()), not_empty: Condvar::new() }
    }

    fn push(&self, t: T) {
        self.items.lock().unwrap().push_back(t);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let guard = self.items.lock().unwrap();
        let mut guard = self.not_empty.wait_while(guard, |q| q.is_empty()).unwrap();
        guard.pop_front().unwrap()
    }
}

fn demo_blocking_queue() {
    println!("-- blocking queue --");

    let q = BlockingQueue::new();
    thread::scope(|s| {
        s.spawn(|| {
            for i in 1..=3 {
                thread::sleep(Duration::from_millis(2));
                q.push(i);
            }
        });

        let got: Vec<i32> = (0..3).map(|_| q.pop()).collect();
        println!("popped in order {got:?}");
    });
}

// Example 5: wait_timeout.
fn demo_wait_timeout() {
    println!("-- wait_timeout --");

    let m = Mutex::new(false);
    let cv = Condvar::new();
    let guard = m.lock().unwrap();
    let (_guard, result) = cv.wait_timeout(guard, Duration::from_millis(5)).unwrap();
    println!("timed out (nobody notified): {}", result.timed_out());
}

// Example 6: Barrier phases.
fn demo_barrier() {
    println!("-- Barrier --");

    let n = 3;
    let bar = Barrier::new(n);
    let arrivals = Mutex::new(Vec::new());
    let leaders = Mutex::new(0);
    thread::scope(|s| {
        for i in 0..n {
            let (bar, arrivals, leaders) = (&bar, &arrivals, &leaders);
            s.spawn(move || {
                arrivals.lock().unwrap().push(format!("phase1 {i}"));
                let r = bar.wait(); // nobody passes until all 3 arrive
                if r.is_leader() {
                    *leaders.lock().unwrap() += 1;
                }

                arrivals.lock().unwrap().push(format!("phase2 {i}"));
            });
        }
    });

    let log = arrivals.into_inner().unwrap();
    let all_phase1_first = log[..n].iter().all(|l| l.starts_with("phase1"));
    println!("all phase1 entries before any phase2: {all_phase1_first}");
    println!("exactly one leader: {}", leaders.into_inner().unwrap() == 1);
}

fn main() {
    demo_send_sync();
    demo_auto_traits();
    demo_condvar();
    demo_blocking_queue();
    demo_wait_timeout();
    demo_barrier();
}
