// 22. Equality, ordering, and hashing
//
// PartialEq gives you == and !=. Eq is a promise that == is a full equivalence
// relation: every value equals itself. Floats are PartialEq but not Eq, because NaN !=
// NaN. PartialOrd gives you <, <=, >, >= and may say "not comparable"; Ord is a total
// order and is what sort() and BTreeMap keys need.
//
// Hash feeds a value into a hasher. HashMap and HashSet keys need Eq + Hash, and the
// two must agree: if a == b then hash(a) == hash(b). If you hand-write one, hand-write
// the other to match.
//
// Run: cargo run --bin 22_eq_ord_hash

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
struct Version(u32, u32);

// Example 1: derived equality and ordering compare field by field.
fn demo_derived() {
    println!("-- derived --");

    let a = Version(1, 2);
    let b = Version(1, 9);
    println!("== {} < {} min {:?}", a == b, a < b, a.min(b));
    println!("cmp {:?} {:?}", a.cmp(&b), b.cmp(&a));

    let mut vs = vec![Version(2, 0), Version(1, 9), Version(1, 2)];
    vs.sort();
    println!("sorted {vs:?}");
}

// Example 2: floats are PartialOrd only.
fn demo_floats() {
    println!("-- floats --");

    let nan = f64::NAN;
    let other_nan = f64::NAN;
    println!("nan == nan {} partial_cmp {:?}", nan == other_nan, nan.partial_cmp(&1.0));
    println!("1.0 vs 2.0 {:?}", 1.0f64.partial_cmp(&2.0));

    // vec![1.0, 0.5].sort();
    // error[E0277]: the trait bound `{float}: Ord` is not satisfied
    let mut fs = vec![2.5, 0.5, 1.5];
    fs.sort_by(f64::total_cmp);
    println!("total_cmp sort {fs:?}");
}

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    // Higher score first; ties broken by name.
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score).then_with(|| self.name.cmp(&other.name))
    }
}

// Example 3: a hand-written ordering.
fn demo_custom_ord() {
    println!("-- custom Ord --");

    let mut players = vec![
        Player { name: "cy".into(), score: 50 },
        Player { name: "ann".into(), score: 90 },
        Player { name: "ben".into(), score: 50 },
    ];
    players.sort();

    for p in &players {
        println!("{} {}", p.name, p.score);
    }

    println!("cy == ben (same score)? {}", players[1] == players[2]);
}

// Example 4: ordering helpers.
fn demo_ordering_helpers() {
    println!("-- Ordering helpers --");

    let words = ["pear", "fig", "banana", "kiwi"];
    let mut by_len = words.to_vec();
    by_len.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    println!("by len then alpha {by_len:?}");

    let mut desc = words.to_vec();
    desc.sort_by(|a, b| b.cmp(a));
    println!("desc {desc:?}");
    println!("reverse {:?} is_lt {}", Ordering::Less.reverse(), 1.cmp(&2).is_lt());
    println!(
        "clamp {} max_by_key {:?}",
        15.clamp(0, 10),
        words.iter().max_by_key(|w| w.len())
    );
}

fn hash_of<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

#[derive(Debug, Eq)]
struct User<'a> {
    id: u64,
    name: &'a str,
}

// Equality and hashing both use only the id, so they agree.
impl PartialEq for User<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for User<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// Example 5: Hash, and keeping it consistent with Eq.
fn demo_hash() {
    println!("-- Hash --");

    println!("same input same hash? {}", hash_of(&"abc") == hash_of(&"abc"));
    println!("different input? {}", hash_of(&"abc") == hash_of(&"abd"));

    let mut roles = HashMap::new();
    roles.insert(User { id: 1, name: "ada" }, "admin");

    let probe = User { id: 1, name: "ignored" };
    println!(
        "lookup by id {:?} (stored name {:?})",
        roles.get(&probe),
        roles.keys().next().map(|u| u.name)
    );
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Suit {
    Clubs,
    Hearts,
}

// Example 6: derived traits make types usable as set and map keys.
fn demo_keys() {
    println!("-- as keys --");

    let mut seen = HashSet::new();
    for s in [Suit::Hearts, Suit::Clubs, Suit::Hearts] {
        if !seen.insert(s) {
            println!("duplicate {s:?}");
        }
    }

    let ordered: BTreeSet<_> = [Suit::Hearts, Suit::Clubs].into_iter().collect();
    println!("ordered by declaration {ordered:?}");

    let pairs: HashSet<(Suit, u8)> =
        [(Suit::Clubs, 1), (Suit::Clubs, 1)].into_iter().collect();
    println!("tuples of Hash types are Hash: {}", pairs.len());
}

fn main() {
    demo_derived();
    demo_floats();
    demo_custom_ord();
    demo_ordering_helpers();
    demo_hash();
    demo_keys();
}
