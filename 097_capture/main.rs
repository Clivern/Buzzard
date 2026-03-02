// 097. Closure capture
//
// Closures capture by reference by default. add mut to the closure if it mutates a
// captured binding. move forces capture by value, which is required when the closure
// outlives the current stack frame, such as a thread.
//
// Run: cargo run --bin 097_capture

fn main() {
    let mut n = 0;
    let mut bump = || {
        n += 1;
        n
    };
    println!("{} {}", bump(), bump());
    let s = String::from("hi");
    let f = move || println!("{s}");
    f();
}
