// 050. Structs
//
// A struct groups named fields. Point { x: 1, y: 2 } sets them by name. Field init
// shorthand Point { x, y } uses local variables of the same name. A let struct starts
// with every field at its value; there is no implicit zero. {:?} needs Debug.
//
//
// Struct update and field shorthand keep constructors short. Debug is for developers;
// add Display when users should see the value.
//
// Run: cargo run --bin 050_structs

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let name = String::from("Ada");
    let age = 36;
    let mut a = Person { name, age };
    a.age = 37;
    println!("{a:?}");
    println!("age {}", a.age);
}
