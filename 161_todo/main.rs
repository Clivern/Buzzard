// 161. todo and unimplemented
//
// todo! panics with a message that this is unfinished. unimplemented! is similar.
// unreachable! means the branch cannot happen; hitting it is a bug. compile_error! fails
// compilation from a macro. Use todo at the sketch stage.
//
// Run: cargo run --bin 161_todo

fn ready() -> i32 {
    1
}

fn main() {
    println!("{}", ready());
    let skip = false;
    if skip {
        unreachable!("skip is false");
    }
}
