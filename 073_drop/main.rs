// 073. Drop
//
// Drop::drop runs when a value goes out of scope. File, String, and Vec use it to free
// resources. You rarely impl Drop unless you wrap a raw resource. drop(v) is a function
// that moves v into an empty scope so it is dropped now. Fields drop after the custom
// drop, in reverse declaration order.
//
// Run: cargo run --bin 073_drop

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let a = Guard("a");
    {
        let _b = Guard("b");
    }
    drop(a);
    println!("after");
}
