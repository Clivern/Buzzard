// 026. Blocks and scope
//
// A name lives until the end of its block. Inner blocks can reuse a name (shadowing).
// When the block ends, owned values are dropped. That drop order is reverse of
// declaration. Braces also group statements in if, match, and loops.
//
// Run: cargo run --bin 026_blocks

fn main() {
    let outer = 1;
    {
        let inner = 2;
        println!("inner {inner} outer {outer}");
    }
    println!("outer {outer}");
}
