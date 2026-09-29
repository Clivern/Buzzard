// 17. Option
//
// Option<T> is Some(T) or None. It replaces null: you cannot use the inner value without
// first deciding what happens when it is missing. match and if let handle both cases
// explicitly.
//
// The combinators keep that handling short. map transforms Some and leaves None alone.
// and_then chains a step that may itself return None. filter keeps Some only when a
// predicate holds. unwrap_or and unwrap_or_else supply a fallback; ok_or turns an Option
// into a Result. ? inside a function returning Option returns None early.
//
// unwrap and expect panic on None. Use them only when None would be a bug.
//
// Run: cargo run --bin 17_option

fn find(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|n| *n == target)
}

// Example 1: producing and matching Options.
fn demo_basics() {
    println!("-- basics --");

    let nums = [1, 2, 3];
    println!("{:?} {:?}", find(&nums, 2), find(&nums, 9));

    for target in [3, 9] {
        match find(&nums, target) {
            Some(i) => println!("{target} at {i}"),
            None => println!("{target} missing"),
        }
    }

    println!("is_some {} is_none {}", Some(1).is_some(), None::<i32>.is_none());
    println!("is_some_and {}", Some(5).is_some_and(|n| n > 3));

    // let n: i32 = Some(5); // error[E0308]: expected `i32`, found `Option<{integer}>`
}

// Example 2: map, and_then, filter.
fn demo_transform() {
    println!("-- map / and_then / filter --");

    let n = Some(2);
    println!("map {:?}", n.map(|x| x * 10));
    println!("and_then {:?}", n.and_then(|x| if x > 0 { Some(x) } else { None }));
    println!("filter {:?} {:?}", n.filter(|x| x % 2 == 0), n.filter(|x| *x > 5));

    // and_then avoids Option<Option<T>>.
    let text = Some("42");
    let nested: Option<Option<i32>> = text.map(|t| t.parse().ok());
    let flat: Option<i32> = text.and_then(|t| t.parse().ok());
    println!("nested {nested:?} flat {flat:?}");
}

// Example 3: getting a value out, with a fallback.
fn demo_fallbacks() {
    println!("-- fallbacks --");

    let none: Option<i32> = None;
    println!("unwrap_or {}", none.unwrap_or(0));
    println!("unwrap_or_else {}", none.unwrap_or_else(|| 40 + 2));
    println!("unwrap_or_default {}", none.unwrap_or_default());
    println!("or {:?} xor {:?}", none.or(Some(9)), Some(1).xor(Some(2)));
    println!("map_or {}", Some(3).map_or(0, |n| n * 2));

    let s: Option<String> = None;
    println!("default string {:?}", s.unwrap_or_default());
    println!("expect {}", Some(7).expect("value present"));
}

// Example 4: borrowing the inside with as_ref, as_mut, and as_deref.
fn demo_borrowing() {
    println!("-- as_ref / as_mut / as_deref --");

    let name: Option<String> = Some(String::from("ada"));
    let len = name.as_ref().map(|s| s.len());
    println!("len {len:?}, name still owned: {name:?}");

    let slice: Option<&str> = name.as_deref();
    println!("as_deref {slice:?}");

    let mut count = Some(1);
    if let Some(c) = count.as_mut() {
        *c += 10;
    }

    println!("as_mut {count:?}");
}

// Example 5: take, replace, insert, get_or_insert.
fn demo_in_place() {
    println!("-- in place --");

    let mut slot = Some(3);
    let old = slot.take();
    println!("take {old:?} left {slot:?}");

    let prev = slot.replace(5);
    println!("replace prev {prev:?} now {slot:?}");

    let mut cache: Option<Vec<i32>> = None;
    cache.get_or_insert_with(Vec::new).push(1);
    cache.get_or_insert_with(Vec::new).push(2);
    println!("get_or_insert_with {cache:?}");
}

struct User {
    name: String,
    manager: Option<Box<User>>,
}

fn manager_name_len(u: &User) -> Option<usize> {
    let m = u.manager.as_ref()?;
    Some(m.name.len())
}

fn first_even_doubled(nums: &[i32]) -> Option<i32> {
    let n = nums.iter().find(|n| *n % 2 == 0)?;
    Some(n * 2)
}

// Example 6: ? on Option.
fn demo_question() {
    println!("-- ? on Option --");

    let boss = User { name: String::from("grace"), manager: None };
    let dev = User { name: String::from("ada"), manager: Some(Box::new(boss)) };
    println!("{} manager len {:?}", dev.name, manager_name_len(&dev));

    let top = dev.manager.as_ref().unwrap();
    println!("{} manager len {:?}", top.name, manager_name_len(top));
    println!("{:?} {:?}", first_even_doubled(&[1, 3, 4]), first_even_doubled(&[1, 3]));
}

// Example 7: converting between Option and Result, and zipping.
fn demo_conversions() {
    println!("-- conversions --");

    let r: Result<i32, &str> = None.ok_or("missing");
    println!("ok_or {r:?}");
    println!("ok {:?}", "5".parse::<i32>().ok());
    println!("zip {:?} {:?}", Some(1).zip(Some("a")), Some(1).zip(None::<&str>));

    let all: Option<Vec<i32>> = [Some(1), Some(2)].into_iter().collect();
    let any_missing: Option<Vec<i32>> = [Some(1), None].into_iter().collect();
    println!("collect {all:?} {any_missing:?}");

    let sum: i32 = [Some(1), None, Some(3)].into_iter().flatten().sum();
    println!("flatten sum {sum}");
}

fn main() {
    demo_basics();
    demo_transform();
    demo_fallbacks();
    demo_borrowing();
    demo_in_place();
    demo_question();
    demo_conversions();
}
