// 085. Default trait methods
//
// A method body in the trait is the default. Implementors can override it. Defaults can
// call other trait methods, which is how Iterator::collect is written in terms of next.
// Keep defaults useful so simple types stay short.
//
// Run: cargo run --bin 085_default_methods

trait Greet {
    fn name(&self) -> &str;
    fn hello(&self) {
        println!("hello {}", self.name());
    }
}

struct User(&'static str);

impl Greet for User {
    fn name(&self) -> &str {
        self.0
    }
}

fn main() {
    User("Ada").hello();
}
