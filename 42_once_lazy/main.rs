// 42. One-time initialization: Once, OnceLock, LazyLock, OnceCell, LazyCell
//
// Globals in Rust must be initialized with a constant expression, so anything that
// needs a heap allocation or a function call at startup uses one of these:
//
// - OnceLock<T>: a thread-safe cell written at most once. get_or_init runs a closure the
//   first time and returns the stored value afterwards.
// - LazyLock<T>: a OnceLock with the closure built in; it runs on first deref. It is the
//   std replacement for the lazy_static crate.
// - Once: runs a side effect at most once (logging setup, C library init). It stores no
//   value.
// - OnceCell and LazyCell: the single-threaded versions, for use inside structs or
//   thread-locals, where no locking is needed.
//
// Run: cargo run --bin 42_once_lazy

use std::cell::{LazyCell, OnceCell};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, Once, OnceLock};
use std::thread;

static START: OnceLock<String> = OnceLock::new();

// Example 1: OnceLock get_or_init.
fn demo_once_lock() {
    println!("-- OnceLock --");

    println!("before init {:?}", START.get());
    let a = START.get_or_init(|| String::from("init"));
    let b = START.get_or_init(|| String::from("ignored"));
    println!("{a} {b} same? {}", std::ptr::eq(a, b));
    println!("set after init {:?}", START.set(String::from("late")));
}

static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn config() -> &'static HashMap<&'static str, &'static str> {
    CONFIG.get_or_init(|| {
        println!("  loading config once");
        HashMap::from([("env", "prod"), ("region", "eu")])
    })
}

// Example 2: a lazily loaded global behind a function.
fn demo_config_accessor() {
    println!("-- config accessor --");

    println!("env {}", config()["env"]);
    println!("region {}", config()["region"]);
}

static NUMS: LazyLock<Vec<i32>> = LazyLock::new(|| {
    println!("  building NUMS");
    (1..=5).map(|n| n * n).collect()
});

static COUNTER: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

// Example 3: LazyLock statics.
fn demo_lazy_lock() {
    println!("-- LazyLock --");

    println!("before first use");
    println!("len {} NUMS[1] {}", NUMS.len(), NUMS[1]); // first deref runs the closure
    println!("again {:?}", *NUMS);
    *COUNTER.lock().unwrap() += 1;
    *COUNTER.lock().unwrap() += 1;
    println!("COUNTER {}", COUNTER.lock().unwrap());
}

static INIT: Once = Once::new();

fn setup_logging() {
    INIT.call_once(|| println!("  logging initialized"));
}

// Example 4: Once for side effects, even across threads.
fn demo_once() {
    println!("-- Once --");

    println!("completed? {}", INIT.is_completed());
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(setup_logging);
        }
    });

    setup_logging();
    println!("completed? {}", INIT.is_completed());
}

// Example 5: many threads racing for one OnceLock.
fn demo_race() {
    println!("-- racing initializers --");

    static WINNER: OnceLock<usize> = OnceLock::new();
    thread::scope(|s| {
        for id in 0..4 {
            s.spawn(move || {
                WINNER.get_or_init(|| id);
            });
        }
    });

    println!("exactly one thread won: {:?}", WINNER.get().is_some());
}

// Example 6: OnceCell inside a struct for a lazily computed field.
struct Report {
    rows: Vec<u32>,
    total: OnceCell<u32>,
}

impl Report {
    fn total(&self) -> u32 {
        *self.total.get_or_init(|| {
            println!("  summing {} rows", self.rows.len());
            self.rows.iter().sum()
        })
    }
}

fn demo_once_cell() {
    println!("-- OnceCell --");

    let r = Report { rows: vec![3, 4, 5], total: OnceCell::new() };
    println!("total {}", r.total());
    println!("total again {}", r.total());
}

// Example 7: LazyCell for a local lazy value.
fn demo_lazy_cell() {
    println!("-- LazyCell --");

    let expensive = LazyCell::new(|| {
        println!("  computing");
        "result".to_uppercase()
    });

    let use_it = true;
    if use_it {
        println!("{} {}", *expensive, expensive.len());
    }

    // static BAD: LazyCell<i32> = LazyCell::new(|| 1);
    // error[E0277]: `UnsafeCell<...>` cannot be shared between threads safely
    // (statics must be Sync; use LazyLock there)
}

fn main() {
    demo_once_lock();
    demo_config_accessor();
    demo_lazy_lock();
    demo_once();
    demo_race();
    demo_once_cell();
    demo_lazy_cell();
}
