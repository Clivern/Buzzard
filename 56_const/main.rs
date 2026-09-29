// 56. const fn, const generics, and compile-time evaluation
//
// A const fn can be called where a value must be known at compile time: const and
// static items, array lengths, and const generic arguments. It can still be called at
// runtime like any function. Inside, you may use arithmetic, if, match, loops, and
// calls to other const fns, but no heap allocation, I/O, or trait methods.
//
// A const generic parameter is a value that is part of a type: struct Grid<const W:
// usize, const H: usize>. [T; N] has always worked this way. One function generic over
// N covers every array length.
//
// const { ... } blocks force an expression to be evaluated at compile time, and a
// failing assert! inside a const turns into a compile error instead of a runtime panic.
//
// Run: cargo run --bin 56_const

const fn square(n: i32) -> i32 {
    n * n
}

const fn factorial(n: u64) -> u64 {
    let mut acc = 1;
    let mut i = 2;
    while i <= n {
        acc *= i;
        i += 1;
    }

    acc
}

const fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }

        d += 1;
    }

    true
}

const N: i32 = square(5);
const FACT_10: u64 = factorial(10);
const BUF_LEN: usize = factorial(4) as usize;
static BUFFER: [u8; BUF_LEN] = [0; BUF_LEN];

// Example 1: const fn at compile time and at runtime.
fn demo_const_fn() {
    println!("-- const fn --");

    println!("N {N} 10! {FACT_10} buffer len {}", BUFFER.len());
    let runtime_input = 7;
    println!("runtime call square({runtime_input}) = {}", square(runtime_input));
    println!("is_prime 97? {} 91? {}", is_prime(97), is_prime(91));

    // const fn bad() -> String { String::from("x") }
    // error[E0015]: cannot call non-const associated function `<String as From<&str>>::from`
    // in constant functions
}

const fn primes<const COUNT: usize>() -> [u32; COUNT] {
    let mut out = [0; COUNT];
    let mut found = 0;
    let mut n = 2;
    while found < COUNT {
        if is_prime(n) {
            out[found] = n;
            found += 1;
        }

        n += 1;
    }

    out
}

const FIRST_PRIMES: [u32; 8] = primes::<8>();

// Example 2: computing a lookup table at compile time.
fn demo_tables() {
    println!("-- compile-time tables --");

    println!("first primes {FIRST_PRIMES:?}");
    const SQUARES: [u32; 6] = {
        let mut t = [0; 6];
        let mut i = 0;
        while i < 6 {
            t[i] = (i * i) as u32;
            i += 1;
        }

        t
    };

    println!("squares {SQUARES:?}");
}

fn first<T, const N: usize>(arr: [T; N]) -> Option<T> {
    arr.into_iter().next()
}

fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    let mut s = 0;
    for v in arr {
        s += v;
    }

    s
}

// Example 3: functions generic over array length.
fn demo_const_generic_fns() {
    println!("-- const generic functions --");

    println!("{:?} {:?}", first([1, 2, 3]), first::<i32, 0>([]));
    println!("sums {} {} {}", sum_array([1, 2]), sum_array([1, 2, 3, 4]), sum_array([]));

    let chunked: Vec<[u8; 2]> =
        [1u8, 2, 3, 4, 5, 6].chunks_exact(2).map(|c| c.try_into().unwrap()).collect();
    println!("as fixed arrays {chunked:?}");
}

#[derive(Debug)]
struct Grid<const W: usize, const H: usize> {
    cells: [[char; W]; H],
}

impl<const W: usize, const H: usize> Grid<W, H> {
    const AREA: usize = W * H;

    fn new() -> Self {
        Grid { cells: [['.'; W]; H] }
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        self.cells[y][x] = c;
    }

    fn render(&self) -> String {
        self.cells
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// Example 4: structs with const parameters.
fn demo_const_generic_structs() {
    println!("-- const generic structs --");

    let mut g: Grid<4, 2> = Grid::new();
    g.set(1, 0, '#');
    g.set(3, 1, '@');
    println!("{}", g.render());
    println!(
        "area {} size of Grid<4,2> {} bytes",
        Grid::<4, 2>::AREA,
        size_of::<Grid<4, 2>>()
    );

    // let a: Grid<4, 2> = Grid::<2, 4>::new();
    // error[E0308]: mismatched types: expected `Grid<4, 2>`, found `Grid<2, 4>`
}

struct Matrix<const R: usize, const C: usize>([[i32; C]; R]);

impl<const R: usize, const K: usize> Matrix<R, K> {
    // The shapes must line up: (R x K) * (K x C) = (R x C), checked by the type system.
    fn mul<const C: usize>(&self, other: &Matrix<K, C>) -> Matrix<R, C> {
        let mut out = [[0; C]; R];
        for (r, row) in out.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = (0..K).map(|k| self.0[r][k] * other.0[k][c]).sum();
            }
        }

        Matrix(out)
    }
}

// Example 5: dimensions checked at compile time.
fn demo_matrix() {
    println!("-- typed matrix --");

    let a = Matrix([[1, 2, 3], [4, 5, 6]]); // 2x3
    let b = Matrix([[1, 0], [0, 1], [1, 1]]); // 3x2
    let c = a.mul(&b); // 2x2
    println!("{:?}", c.0);

    // b.mul(&b); // error[E0308]: mismatched types (3x2 times 3x2 does not line up)
}

// Example 6: const blocks and compile-time assertions.
const _: () = assert!(size_of::<u64>() == 8, "this code assumes 64-bit integers");

struct Aligned<const N: usize>;

impl<const N: usize> Aligned<N> {
    const CHECK: () = assert!(N.is_power_of_two(), "N must be a power of two");
    fn mask() -> usize {
        let () = Self::CHECK; // evaluated at compile time for each N used
        N - 1
    }
}

fn demo_const_blocks() {
    println!("-- const blocks --");

    let v = const { factorial(5) * 2 };
    println!("const block value {v}");

    let empty: [Vec<i32>; 3] = [const { Vec::new() }; 3]; // Vec is not Copy, const block is fine
    println!("array of empty Vecs {empty:?}");
    println!("mask for 16: {:#x}", Aligned::<16>::mask());

    // Aligned::<12>::mask();
    // error[E0080]: evaluation panicked: N must be a power of two
}

fn main() {
    demo_const_fn();
    demo_tables();
    demo_const_generic_fns();
    demo_const_generic_structs();
    demo_matrix();
    demo_const_blocks();
}
