// 021. Functions
//
// fn names a function. Parameters need types. The return type follows ->. The last
// expression in a block is the return value when it has no semicolon. return exits
// early. A function with no -> returns ().
//
// Run: cargo run --bin 021_functions

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn swap(a: String, b: String) -> (String, String) {
    (b, a)
}

fn main() {
    println!("{}", add(2, 3));
    let (left, right) = swap(String::from("left"), String::from("right"));
    println!("{left} {right}");
}
