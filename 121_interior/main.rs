// 121. Interior mutability
//
// Usually mutation needs &mut. Interior mutability types (Cell, RefCell, Mutex, atomic
// integers) let you mutate through &self. They move the uniqueness check to runtime or
// to hardware atomics. Reach for them when a struct must mutate a field while shared.
//
// Run: cargo run --bin 121_interior

use std::cell::RefCell;

struct Cache {
    hits: RefCell<u32>,
}

impl Cache {
    fn hit(&self) {
        *self.hits.borrow_mut() += 1;
    }
}

fn main() {
    let c = Cache {
        hits: RefCell::new(0),
    };
    c.hit();
    c.hit();
    println!("{}", c.hits.borrow());
}
