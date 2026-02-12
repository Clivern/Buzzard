// 038. let else
//
// let pattern = expr else { diverging block }; continues with the bindings if the
// pattern matches. The else block must not return to this function: it returns, panics,
// or breaks. It is handy at the top of a function to reject a bad input without nesting.
//
// Run: cargo run --bin 038_let_else

fn greet(name: Option<&str>) {
    let Some(name) = name else {
        println!("no name");
        return;
    };
    println!("hello {name}");
}

fn main() {
    greet(Some("Ada"));
    greet(None);
}
