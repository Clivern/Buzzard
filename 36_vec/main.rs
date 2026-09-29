// 36. Vec, sorting, and binary search
//
// Vec<T> is a growable array on the heap: a pointer, a length, and a capacity. push and
// pop work at the end in O(1). Capacity is room already reserved; with_capacity and
// reserve avoid repeated reallocations when you know the size. Indexing v[i] panics on a
// bad index; get(i) returns an Option.
//
// sort needs Ord and is stable (equal items keep their order). sort_by takes a
// comparator, sort_by_key a key function, and the _unstable variants are faster but may
// reorder equal items. binary_search on a sorted slice returns Ok(index) when found or
// Err(insert_position) when not.
//
// Run: cargo run --bin 36_vec

use std::cmp::Reverse;

// Example 1: creating, pushing, popping, and capacity.
fn demo_basics() {
    println!("-- basics --");

    let mut v = Vec::with_capacity(2);
    println!("empty len {} cap {}", v.len(), v.capacity());
    v.push(1);
    v.push(2);
    v.push(3); // exceeds capacity, so it reallocates
    println!("len {} cap>=3 {} {:?}", v.len(), v.capacity() >= 3, v);
    println!("pop {:?} first {:?} last {:?}", v.pop(), v.first(), v.last());

    let zeros = vec![0; 5];
    let from_iter: Vec<_> = (1..=3).collect();
    println!("{zeros:?} {from_iter:?}");
    v.shrink_to_fit();
    println!("after shrink cap {}", v.capacity());
}

// Example 2: safe and unsafe-looking access.
fn demo_access() {
    println!("-- access --");

    let mut v = vec![10, 20, 30];
    println!("v[1] {} get(9) {:?}", v[1], v.get(9));

    if let Some(x) = v.get_mut(0) {
        *x += 1;
    }

    println!("after get_mut {v:?}");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // hide the panic message
    let caught = std::panic::catch_unwind(|| vec![1][5]);
    std::panic::set_hook(hook);
    println!("out of range index panics: {}", caught.is_err());
    println!("contains 20? {} is_empty? {}", v.contains(&20), v.is_empty());
}

// Example 3: inserting and removing in the middle.
fn demo_insert_remove() {
    println!("-- insert / remove --");

    let mut v = vec!['a', 'b', 'd'];
    v.insert(2, 'c');
    println!("insert {v:?}");

    let removed = v.remove(0); // O(n): shifts everything left
    println!("remove {removed} -> {v:?}");

    let swapped = v.swap_remove(0); // O(1): moves the last item into the gap
    println!("swap_remove {swapped} -> {v:?}");
    v.truncate(1);
    println!("truncate {v:?}");
    v.clear();
    println!("clear {v:?}");
}

// Example 4: bulk operations.
fn demo_bulk() {
    println!("-- bulk --");

    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|n| n % 2 == 0);
    println!("retain even {v:?}");
    v.extend([8, 10]);
    v.extend_from_slice(&[12]);
    println!("extend {v:?}");

    let drained: Vec<_> = v.drain(1..3).collect();
    println!("drained {drained:?} left {v:?}");

    let tail = v.split_off(2);
    println!("split_off {v:?} {tail:?}");

    let mut a = vec![1, 2];
    let mut b = vec![3, 4];
    a.append(&mut b);
    println!("append {a:?} b now {b:?}");

    let mut d = vec![1, 1, 2, 3, 3, 3, 1];
    d.dedup();
    println!("dedup consecutive {d:?}");

    let mut words = vec!["a", "b", "c"];
    words.reverse();
    println!("{} {:?}", words.concat(), words.iter().rev().collect::<Vec<_>>());
}

// Example 5: sorting.
fn demo_sorting() {
    println!("-- sorting --");

    let mut names = vec!["bob", "Cy", "ann"];
    names.sort();
    println!("byte order {names:?}");
    names.sort_by_key(|s| s.to_lowercase());
    println!("case-insensitive {names:?}");

    let mut pairs = vec![("ke", 80), ("joe", 20), ("amy", 80)];
    pairs.sort_by_key(|p| Reverse(p.1));
    println!("by score desc, stable {pairs:?}");
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("score desc then name {pairs:?}");

    let mut nums = vec![5, 3, 9, 1, 7];
    nums.sort_unstable();
    println!("unstable {nums:?} is_sorted {}", nums.is_sorted());

    let mut floats = vec![2.5, -1.0, 0.0];
    floats.sort_by(f64::total_cmp);
    println!("floats {floats:?}");

    // Partial sort: the 3 smallest, without sorting the rest.
    let mut big = vec![9, 4, 7, 1, 8, 2, 6];
    let (below, third, _) = big.select_nth_unstable(2);
    let mut smallest = below.to_vec();
    smallest.push(*third);
    smallest.sort();
    println!("three smallest {smallest:?}");
}

// Example 6: binary search on sorted data.
fn demo_binary_search() {
    println!("-- binary search --");

    let v = [1, 3, 5, 7];
    println!("{:?} {:?}", v.binary_search(&5), v.binary_search(&4));

    // Insert while keeping the Vec sorted.
    let mut sorted = vec![10, 20, 40];
    let pos = sorted.binary_search(&30).unwrap_or_else(|i| i);
    sorted.insert(pos, 30);
    println!("inserted at {pos}: {sorted:?}");

    let people = [("ann", 30), ("ben", 35), ("cy", 41)];
    println!("by key {:?}", people.binary_search_by_key(&35, |p| p.1));
    println!("partition_point first >= 33: {}", people.partition_point(|p| p.1 < 33));

    // Searching unsorted data gives a meaningless answer, not an error.
}

// Example 7: Vec of structs, and Vec<Vec<T>>.
fn demo_nested() {
    println!("-- nested --");

    let mut grid = vec![vec![0; 3]; 2];
    grid[1][2] = 7;
    println!("{grid:?}");
    #[derive(Debug)]
    struct Task {
        id: u32,
        done: bool,
    }

    let mut tasks: Vec<Task> =
        (1..=4).map(|id| Task { id, done: id % 2 == 0 }).collect();
    tasks.retain(|t| !t.done);
    println!("open tasks {:?}", tasks.iter().map(|t| t.id).collect::<Vec<_>>());
    println!("{:?}", tasks.first());
}

fn main() {
    demo_basics();
    demo_access();
    demo_insert_remove();
    demo_bulk();
    demo_sorting();
    demo_binary_search();
    demo_nested();
}
