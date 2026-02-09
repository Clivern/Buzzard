// 025. The never type
//
// ! is the never type, used for functions that do not return: panic!, unimplemented!,
// loop {}. You can assign never to any type because there is no value to assign. match
// branches can mix a real value with panic! on the impossible arm.
//
// Run: cargo run --bin 025_diverging

fn halt(msg: &str) -> ! {
    panic!("{msg}");
}

fn pick(flag: bool) -> i32 {
    if flag {
        7
    } else {
        halt("flag was false");
    }
}

fn main() {
    println!("{}", pick(true));
}
