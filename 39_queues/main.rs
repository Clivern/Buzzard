// 39. VecDeque, BinaryHeap, and LinkedList
//
// VecDeque is a double-ended queue backed by a ring buffer: push and pop at both the
// front and the back in O(1). Use it for FIFO queues, breadth-first search, and sliding
// windows. Index 0 is the front.
//
// BinaryHeap is a max-heap priority queue: peek is the largest item, pop removes it,
// and push is O(log n). Wrap items in std::cmp::Reverse for a min-heap. into_sorted_vec
// consumes it into ascending order.
//
// LinkedList is a doubly linked list. It is rarely the right choice: Vec and VecDeque
// are faster for almost everything because their memory is contiguous. Its niche is
// O(1) append and split of whole lists.
//
// Run: cargo run --bin 39_queues

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, LinkedList, VecDeque};

// Example 1: VecDeque at both ends.
fn demo_vecdeque() {
    println!("-- VecDeque --");

    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_front(0);
    println!("{q:?} front {:?} back {:?} q[1] {}", q.front(), q.back(), q[1]);
    println!("pop_front {:?} pop_back {:?} left {q:?}", q.pop_front(), q.pop_back());

    let mut r: VecDeque<_> = (1..=5).collect();
    r.rotate_left(2);
    println!("rotated {r:?}");

    let contiguous: &[i32] = r.make_contiguous();
    println!("as slice {contiguous:?}");
}

// Example 2: a FIFO work queue.
fn demo_work_queue() {
    println!("-- FIFO queue --");

    let mut jobs: VecDeque<&str> = ["build", "test"].into();
    jobs.push_back("deploy");

    while let Some(job) = jobs.pop_front() {
        println!("run {job}");
        if job == "test" {
            jobs.push_back("report");
        }
    }
}

// Example 3: breadth-first search on a small graph.
fn demo_bfs() {
    println!("-- BFS --");

    let edges = [("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("d", "e")];
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (from, to) in edges {
        graph.entry(from).or_default().push(to);
    }

    let mut dist: HashMap<&str, u32> = HashMap::from([("a", 0)]);
    let mut queue = VecDeque::from(["a"]);
    while let Some(node) = queue.pop_front() {
        for &next in graph.get(node).into_iter().flatten() {
            if !dist.contains_key(next) {
                dist.insert(next, dist[node] + 1);
                queue.push_back(next);
            }
        }
    }

    let mut out: Vec<_> = dist.into_iter().collect();
    out.sort();
    println!("hops from a {out:?}");
}

// Example 4: a sliding window maximum with VecDeque.
fn demo_sliding_window() {
    println!("-- sliding window --");

    let v = [1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let mut window: VecDeque<usize> = VecDeque::new(); // indices, values decreasing
    let mut maxes = Vec::new();
    for i in 0..v.len() {
        while window.back().is_some_and(|&j| v[j] <= v[i]) {
            window.pop_back();
        }

        window.push_back(i);
        if window[0] + k <= i {
            window.pop_front();
        }

        if i + 1 >= k {
            maxes.push(v[window[0]]);
        }
    }

    println!("max of each window of {k}: {maxes:?}");
}

// Example 5: BinaryHeap as max-heap and min-heap.
fn demo_binary_heap() {
    println!("-- BinaryHeap --");

    let mut h = BinaryHeap::from([3, 1, 4, 1, 5]);
    println!("peek {:?} len {}", h.peek(), h.len());
    h.push(9);

    let mut popped = Vec::new();
    while let Some(x) = h.pop() {
        popped.push(x);
    }

    println!("pops largest first {popped:?}");

    let mut min = BinaryHeap::new();
    for n in [3, 1, 2] {
        min.push(Reverse(n));
    }

    println!("min-heap pop {:?}", min.pop().map(|Reverse(n)| n));
    println!("into_sorted_vec {:?}", BinaryHeap::from([5, 2, 8]).into_sorted_vec());

    // Top-k smallest by keeping a bounded max-heap.
    let data = [9, 4, 7, 1, 8, 2];
    let mut top = BinaryHeap::new();
    for &d in &data {
        top.push(d);
        if top.len() > 3 {
            top.pop();
        }
    }

    println!("3 smallest {:?}", top.into_sorted_vec());
}

// Example 6: a priority queue of tasks.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Task {
    priority: u8,
    name: &'static str,
}

fn demo_priority_queue() {
    println!("-- priority tasks --");

    let mut q = BinaryHeap::new();
    q.push(Task { priority: 2, name: "email" });

    q.push(Task { priority: 9, name: "outage" });

    q.push(Task { priority: 5, name: "review" });

    while let Some(Task { priority, name }) = q.pop() {
        println!("p{priority} {name}");
    }

    // Dijkstra's algorithm uses exactly this, with Reverse((distance, node)).
}

// Example 7: LinkedList, for completeness.
fn demo_linked_list() {
    println!("-- LinkedList --");

    let mut a = LinkedList::from([1, 2]);
    let mut b = LinkedList::from([3, 4]);
    a.append(&mut b); // O(1)
    a.push_front(0);
    println!("{a:?} b empty? {}", b.is_empty());

    let tail = a.split_off(3);
    println!("split {a:?} {tail:?}");
}

fn main() {
    demo_vecdeque();
    demo_work_queue();
    demo_bfs();
    demo_sliding_window();
    demo_binary_heap();
    demo_priority_queue();
    demo_linked_list();
}
