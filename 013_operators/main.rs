// 013. Operators
//
// 7/2 is integer division, so the result is 3. 7.0/2.0 keeps the fraction. % is the
// remainder. << and >> shift bits. &, |, and ^ are bitwise and, or, and xor. Compound
// assignment such as += updates a mut binding.
//
// Run: cargo run --bin 013_operators

fn main() {
    println!("int div {} remainder {} float {}", 7 / 2, 7 % 2, 7.0 / 2.0);
    println!("shift {} {} bits {} {} {}", 1 << 3, 8 >> 1, 5 & 3, 5 | 2, 5 ^ 1);
    let mut n = 1;
    n += 4;
    println!("n {n}");
}
