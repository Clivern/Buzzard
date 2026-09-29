// shapes::circle, declared with `pub mod circle;` in shapes.rs.

pub struct Circle {
    pub r: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        // super:: is the parent module (shapes), which can see its private helpers.
        super::helpers::round2(std::f64::consts::PI * self.r * self.r)
    }
}
