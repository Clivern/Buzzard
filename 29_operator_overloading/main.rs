// 29. Operator overloading and indexing
//
// Operators are trait methods from std::ops. a + b calls Add::add(a, b); a += b calls
// AddAssign; -a calls Neg; v[i] calls Index (or IndexMut when you assign). Implement
// the trait for your type and the operator works. Output is an associated type, so the
// result can be a different type than the inputs. You cannot invent new operators.
//
// Comparison operators come from PartialEq and PartialOrd (lesson 22), not std::ops.
//
// Index returns a reference into the container, so v[i] can be read or, with IndexMut,
// assigned. Vec and slices panic on a bad index; use get when the index might be out of
// range.
//
// Run: cargo run --bin 29_operator_overloading

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Watts(u32);

impl Add for Watts {
    type Output = Watts;
    fn add(self, rhs: Watts) -> Watts {
        Watts(self.0 + rhs.0)
    }
}

// Example 1: a single binary operator.
fn demo_add() {
    println!("-- Add --");

    let w = Watts(40) + Watts(20);
    println!("{w:?}");

    // Watts(1) - Watts(1); // error[E0369]: binary operation `-` cannot be applied
    let total =
        [Watts(5), Watts(10), Watts(15)].into_iter().fold(Watts(0), |a, b| a + b);
    println!("folded {total:?}");
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct V2 {
    x: f64,
    y: f64,
}

impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        V2 { x: self.x + o.x, y: self.y + o.y }
    }
}

impl Sub for V2 {
    type Output = V2;
    fn sub(self, o: V2) -> V2 {
        V2 { x: self.x - o.x, y: self.y - o.y }
    }
}

impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        V2 { x: -self.x, y: -self.y }
    }
}

// Vector times scalar: the right-hand side is a different type.
impl Mul<f64> for V2 {
    type Output = V2;
    fn mul(self, k: f64) -> V2 {
        V2 { x: self.x * k, y: self.y * k }
    }
}

// Scalar times vector needs its own impl, on f64.
impl Mul<V2> for f64 {
    type Output = V2;
    fn mul(self, v: V2) -> V2 {
        v * self
    }
}

// Dot product: V2 * V2 gives a f64.
impl Mul for V2 {
    type Output = f64;
    fn mul(self, o: V2) -> f64 {
        self.x * o.x + self.y * o.y
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, o: V2) {
        self.x += o.x;
        self.y += o.y;
    }
}

// Example 2: a full set of vector operators.
fn demo_vector_math() {
    println!("-- vector math --");

    let a = V2 { x: 1.0, y: 2.0 };
    let b = V2 { x: 3.0, y: -1.0 };
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("-a = {:?}", -a);
    println!("a * 2 = {:?} 2 * a = {:?}", a * 2.0, 2.0 * a);
    println!("a . b = {}", a * b);

    let mut pos = V2::default();
    for _ in 0..3 {
        pos += a;
    }

    println!("after += x3 {pos:?}");
}

// Example 3: operators on references, to avoid moving non-Copy values.
#[derive(Debug, Clone, PartialEq)]
struct Poly(Vec<i32>);

impl Add for &Poly {
    type Output = Poly;
    fn add(self, o: &Poly) -> Poly {
        let n = self.0.len().max(o.0.len());
        Poly(
            (0..n)
                .map(|i| self.0.get(i).unwrap_or(&0) + o.0.get(i).unwrap_or(&0))
                .collect(),
        )
    }
}

fn demo_ref_operators() {
    println!("-- operators on references --");

    let p = Poly(vec![1, 2, 3]);
    let q = Poly(vec![10, 20]);
    let r = &p + &q;
    println!("{r:?} and p, q still usable: {p:?} {q:?}");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Flags(u8);

impl Not for Flags {
    type Output = Flags;
    fn not(self) -> Flags {
        Flags(!self.0)
    }
}

// Example 4: unary Not.
fn demo_not() {
    println!("-- Not --");

    let f = Flags(0b0000_1111);
    println!("{:08b} -> {:08b}", f.0, (!f).0);
}

struct Grid {
    w: usize,
    cells: Vec<char>,
}

impl Index<(usize, usize)> for Grid {
    type Output = char;
    fn index(&self, (row, col): (usize, usize)) -> &char {
        &self.cells[row * self.w + col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut char {
        &mut self.cells[row * self.w + col]
    }
}

// Example 5: Index and IndexMut with a custom index type.
fn demo_index() {
    println!("-- Index / IndexMut --");

    let v = vec![10, 20, 30];
    println!("{} {:?} {:?}", v[1], &v[1..], v.get(9));

    let mut g = Grid { w: 3, cells: vec!['.'; 6] };
    g[(0, 1)] = '#';
    g[(1, 2)] = '@';

    for row in 0..2 {
        let line: String = (0..3).map(|c| g[(row, c)]).collect();
        println!("{line}");
    }

    let m: std::collections::HashMap<&str, i32> = [("a", 1)].into();
    println!("map index {}", m["a"]); // panics if the key is missing; prefer get
}

// Example 6: Index by an enum key.
#[derive(Clone, Copy)]
enum Channel {
    Red,
    Green,
    Blue,
}

struct Pixel([u8; 3]);

impl Index<Channel> for Pixel {
    type Output = u8;
    fn index(&self, c: Channel) -> &u8 {
        &self.0[c as usize]
    }
}

fn demo_enum_index() {
    println!("-- enum index --");

    let p = Pixel([255, 128, 0]);
    println!("r {} g {} b {}", p[Channel::Red], p[Channel::Green], p[Channel::Blue]);
}

fn main() {
    demo_add();
    demo_vector_math();
    demo_ref_operators();
    demo_not();
    demo_index();
    demo_enum_index();
}
