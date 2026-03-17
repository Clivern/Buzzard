// 152. Formatting extras
//
// {:08} pads. {:x} hex. {:.2} two decimal places. {:>6} right-align. Debug of a custom
// type needs #[derive(Debug)] or a manual impl. format_args! is the lazy version used by
// println!.
//
// Run: cargo run --bin 152_fmt_extras

fn main() {
    println!("{:08} {:x} {:.2} {:>6}", 42, 255, 3.14159, "ada");
}
