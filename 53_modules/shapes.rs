// The shapes module. Because this file is shapes.rs, its submodules live in shapes/.

pub mod circle;

mod helpers {
    // Private to shapes: only code inside shapes (and its children) can call it.
    pub fn round2(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }
}

pub struct Square {
    pub side: f64,
}

impl Square {
    pub fn area(&self) -> f64 {
        helpers::round2(self.side * self.side)
    }
}

// Re-export so callers can write shapes::Circle instead of shapes::circle::Circle.
pub use circle::Circle;
