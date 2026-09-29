// 44. Shared state: Arc, Mutex, RwLock, and atomics
//
// Arc<T> is Rc for threads: an atomically reference-counted pointer. Arc gives shared
// read-only access; to mutate, put something inside that allows it.
//
// Mutex<T> allows one thread at a time. lock() blocks until it gets a MutexGuard, which
// derefs to T and unlocks when dropped, so the guard's scope is the critical section.
// If a thread panics while holding the lock, the mutex is poisoned and lock() returns
// Err from then on. RwLock<T> allows many readers or one writer; it pays off for
// read-heavy data.
//
// Atomic integers (AtomicUsize, AtomicBool, ...) are lock-free shared values. Each
// operation takes an Ordering; SeqCst is the simplest to reason about, and Relaxed is
// enough for a counter whose only job is the final total.
//
// Prefer channels (lesson 45) when threads can just send each other values.
//
// Run: cargo run --bin 44_shared_state

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// Example 1: Arc shares read-only data.
fn demo_arc() {
    println!("-- Arc --");

    let words = Arc::new(vec!["a", "bb", "ccc"]);
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let words = Arc::clone(&words);
            thread::spawn(move || words[i].len())
        })
        .collect();

    let lens: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("lens {lens:?} strong count back to {}", Arc::strong_count(&words));

    // let rc = std::rc::Rc::new(1);
    // thread::spawn(move || println!("{rc}"));
    // error[E0277]: `Rc<i32>` cannot be sent between threads safely
}

// Example 2: Arc<Mutex<T>> counter.
fn demo_mutex() {
    println!("-- Mutex --");

    let n = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let n = Arc::clone(&n);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *n.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("total {}", n.lock().unwrap());
}

// Example 3: keep the critical section short.
fn demo_guard_scope() {
    println!("-- guard scope --");

    let log = Mutex::new(Vec::new());
    thread::scope(|s| {
        for id in 0..3 {
            let log = &log;
            s.spawn(move || {
                let line = format!("worker {id} done"); // work outside the lock
                log.lock().unwrap().push(line); // guard dropped at end of statement
            });
        }
    });

    let mut lines = log.into_inner().unwrap();
    lines.sort();
    println!("{lines:?}");

    let m = Mutex::new(5);
    {
        let mut g = m.lock().unwrap();
        *g += 1;
        println!("try_lock while held: {}", m.try_lock().is_err());
    }

    println!("try_lock after drop: {}", m.try_lock().is_ok());
}

// Example 4: poisoning.
fn demo_poison() {
    println!("-- poisoning --");

    let m = Arc::new(Mutex::new(vec![1, 2]));
    let m2 = Arc::clone(&m);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let _ = thread::spawn(move || {
        let _g = m2.lock().unwrap();
        panic!("died holding the lock");
    })
    .join();
    std::panic::set_hook(hook);
    println!("poisoned? {}", m.is_poisoned());

    // The data is still there if you decide it is safe to use.
    let data = m.lock().unwrap_or_else(|e| e.into_inner());
    println!("recovered {data:?}");
}

// Example 5: RwLock for read-heavy data.
fn demo_rwlock() {
    println!("-- RwLock --");

    let cache = RwLock::new(HashMap::from([("a", 1)]));
    {
        let r1 = cache.read().unwrap();
        let r2 = cache.read().unwrap();
        println!("two readers at once: {:?} {:?}", r1.get("a"), r2.get("a"));
    }

    cache.write().unwrap().insert("b", 2);
    let hits = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                if cache.read().unwrap().contains_key("b") {
                    hits.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    println!("readers that saw b: {}", hits.load(Ordering::Relaxed));
}

// Example 6: atomics.
fn demo_atomics() {
    println!("-- atomics --");

    let n = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1000 {
                    n.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    println!("count {}", n.load(Ordering::SeqCst));

    let max = AtomicU64::new(0);
    thread::scope(|s| {
        for v in [17, 42, 8, 23] {
            let max = &max;
            s.spawn(move || max.fetch_max(v, Ordering::Relaxed));
        }
    });

    println!("fetch_max {}", max.load(Ordering::Relaxed));

    let x = AtomicUsize::new(5);
    println!(
        "swap old {} cas {:?} {:?}",
        x.swap(6, Ordering::SeqCst),
        x.compare_exchange(6, 7, Ordering::SeqCst, Ordering::SeqCst),
        x.compare_exchange(6, 8, Ordering::SeqCst, Ordering::SeqCst)
    );
}

// Example 7: an AtomicBool stop flag.
fn demo_stop_flag() {
    println!("-- stop flag --");

    let stop = Arc::new(AtomicBool::new(false));
    let worker = {
        let stop = Arc::clone(&stop);
        thread::spawn(move || {
            let mut loops = 0u64;
            while !stop.load(Ordering::Acquire) {
                loops += 1;
                thread::yield_now();
            }

            loops
        })
    };

    thread::sleep(std::time::Duration::from_millis(5));
    stop.store(true, Ordering::Release);
    println!("worker stopped after some loops: {}", worker.join().unwrap() > 0);
}

fn main() {
    demo_arc();
    demo_mutex();
    demo_guard_scope();
    demo_poison();
    demo_rwlock();
    demo_atomics();
    demo_stop_flag();
}
