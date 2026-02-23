// 071. Eq and Ord
//
// PartialEq is ==. Eq promises == is an equivalence relation (floats are not Eq because
// NaN). PartialOrd is < and friends. Ord is a total order, needed by BTreeMap keys.
// #[derive] them when field-by-field comparison is what you want.
//
// Run: cargo run --bin 071_eq_ord

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Version(u32, u32);

fn main() {
    let a = Version(1, 2);
    let b = Version(1, 9);
    println!("{} {:?}", a == b, a.min(b));
}
