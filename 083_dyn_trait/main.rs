// 083. Trait objects
//
// dyn Trait is a type that can hold any implementor, with a vtable for the methods. It
// must be behind a pointer: Box<dyn Trait>, &dyn Trait. The trait must be object-safe:
// no generic methods, and Self cannot be returned by value unless the trait is ?Sized-
// aware. Use dyn when you need mixed types at runtime.
//
// Run: cargo run --bin 083_dyn_trait

trait Speak {
    fn speak(&self) -> &'static str;
}

struct Dog;
struct Robot;

impl Speak for Dog {
    fn speak(&self) -> &'static str {
        "woof"
    }
}
impl Speak for Robot {
    fn speak(&self) -> &'static str {
        "beep"
    }
}

fn say(s: &dyn Speak) {
    println!("{}", s.speak());
}

fn main() {
    say(&Dog);
    say(&Robot);
    let list: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Robot)];
    for s in list {
        println!("{}", s.speak());
    }
}
