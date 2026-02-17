// 054. Methods
//
// impl Block attaches methods to a type. &self borrows. &mut self mutates. self takes
// ownership. A method is called with a dot: c.inc(). The compiler auto-references and
// auto-dereferences the receiver so the call still works on a value or a pointer.
//
// Run: cargo run --bin 054_methods

struct Counter {
    n: i32,
}

impl Counter {
    fn value(&self) -> i32 {
        self.n
    }

    fn inc(&mut self) {
        self.n += 1;
    }
}

fn main() {
    let mut c = Counter { n: 0 };
    c.inc();
    c.inc();
    println!("{}", c.value());
}
