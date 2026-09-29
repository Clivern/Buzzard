// 43. Threads: spawn, move, Builder, scoped threads, thread_local, park
//
// thread::spawn starts an OS thread running a closure and returns a JoinHandle. join()
// waits for it and returns the closure's result, or Err if it panicked. When main
// returns, the process exits and any unjoined threads are killed.
//
// The closure passed to spawn must be 'static and Send: it may outlive the caller, so
// it must own everything it uses. move transfers captured values into it; clone an Arc
// first when several threads need the same data. thread::scope lifts the 'static
// requirement: threads spawned in a scope may borrow locals, because the scope joins
// them all before it returns.
//
// Builder sets a thread's name and stack size. thread_local! gives every thread its own
// copy of a value. park and unpark are a low-level sleep/wake building block.
//
// Run: cargo run --bin 43_threads

use std::cell::{Cell, RefCell};
use std::thread;
use std::time::Duration;

// Example 1: spawn and join.
fn demo_spawn() {
    println!("-- spawn / join --");

    let h = thread::spawn(|| {
        println!("in a thread");
        7
    });

    println!("in main");
    println!("joined {}", h.join().unwrap());

    let handles: Vec<_> = (0..4).map(|i| thread::spawn(move || i * i)).collect();
    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("results in spawn order {results:?}");
}

// Example 2: move closures.
fn demo_move() {
    println!("-- move --");

    let s = String::from("hello");

    // thread::spawn(|| println!("{s}"));
    // error[E0373]: closure may outlive the current function, but it borrows `s`
    let h = thread::spawn(move || format!("{s} from a thread"));
    println!("{}", h.join().unwrap());

    let data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at(3);
    let (left, right) = (left.to_vec(), right.to_vec());
    let a = thread::spawn(move || left.iter().sum::<i32>());
    let b = thread::spawn(move || right.iter().sum::<i32>());
    println!("parallel sum {}", a.join().unwrap() + b.join().unwrap());
}

// Example 3: a panicking thread does not take down main.
fn demo_panic_in_thread() {
    println!("-- panic in a thread --");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let h = thread::spawn(|| {
        if true {
            panic!("worker failed");
        }

        1
    });

    let r = h.join();
    std::panic::set_hook(hook);
    println!("join returned Err? {}", r.is_err());
}

// Example 4: Builder, names, and ids.
fn demo_builder() {
    println!("-- Builder --");

    let h = thread::Builder::new()
        .name(String::from("worker-1"))
        .stack_size(64 * 1024)
        .spawn(|| thread::current().name().map(str::to_owned))
        .unwrap();

    println!("thread name {:?}", h.join().unwrap());
    println!("main is {:?}", thread::current().name());

    let ids: Vec<_> = (0..2).map(|_| thread::spawn(|| thread::current().id())).collect();
    let ids: Vec<_> = ids.into_iter().map(|h| h.join().unwrap()).collect();
    println!("distinct ids? {}", ids[0] != ids[1]);
    println!("available parallelism {:?}", thread::available_parallelism().is_ok());
}

// Example 5: scoped threads borrow local data.
fn demo_scoped() {
    println!("-- scoped threads --");

    let v = vec![1, 2, 3];
    let mut total = 0;
    thread::scope(|s| {
        s.spawn(|| println!("borrowed {:?}", &v));
        s.spawn(|| println!("len {}", v.len()));

        let h = s.spawn(|| v.iter().sum::<i32>());
        total = h.join().unwrap();
    });

    println!("total {total}, v still owned here: {v:?}");

    // Each thread gets its own mutable chunk.
    let mut data = [1, 2, 3, 4, 5, 6, 7, 8];
    thread::scope(|s| {
        for chunk in data.chunks_mut(3) {
            s.spawn(move || chunk.iter_mut().for_each(|x| *x *= 10));
        }
    });

    println!("chunks doubled in parallel {data:?}");
}

thread_local! {
    static COUNT: Cell<u32> = const { Cell::new(0) };
    static SCRATCH: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

// Example 6: thread_local values.
fn demo_thread_local() {
    println!("-- thread_local --");

    COUNT.set(COUNT.get() + 1);
    COUNT.with(|c| c.set(c.get() + 1));
    println!("main count {}", COUNT.get());

    let other = thread::spawn(|| {
        COUNT.set(COUNT.get() + 100);
        COUNT.get()
    });

    println!("other thread count {} (its own copy)", other.join().unwrap());
    println!("main count still {}", COUNT.get());
    SCRATCH.with_borrow_mut(|v| v.extend([1, 2, 3]));
    println!("scratch {:?}", SCRATCH.with_borrow(|v| v.clone()));
}

// Example 7: park and unpark.
fn demo_park() {
    println!("-- park / unpark --");

    let main_thread = thread::current();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        main_thread.unpark();
    });

    thread::park(); // returns once unparked (or spuriously; real code loops on a flag)
    println!("unparked");

    // unpark before park leaves a token, so the next park returns right away.
    thread::current().unpark();
    thread::park();
    println!("token consumed without blocking");
    thread::park_timeout(Duration::from_millis(1));
    println!("park_timeout returned");
    thread::yield_now();
}

fn main() {
    demo_spawn();
    demo_move();
    demo_panic_in_thread();
    demo_builder();
    demo_scoped();
    demo_thread_local();
    demo_park();
}
