//! Math types and helpers.
//!
//! Consists of re-exported `glam` types with some additions.



pub use tetra::math::*; 

mod circle;
mod rect;

pub use circle::Circle;
pub use rect::{Rectangle, RectOffset};


pub fn vec3(x: f32, y: f32, z: f32) -> Vec3<f32> {
    Vec3::new(x, y, z)
}

/// Converts 2d polar coordinates to 2d cartesian coordinates.
pub fn polar_to_cartesian(rho: f32, theta: f32) -> Vec2<f32> {
    Vec2::new(rho * theta.cos(), rho * theta.sin())
}

/// Converts 2d cartesian coordinates to 2d polar coordinates.
pub fn cartesian_to_polar(cartesian: Vec2<f32>) -> Vec2<f32> {
    Vec2::new(
        (cartesian.x.powi(2) + cartesian.y.powi(2)).sqrt(),
        cartesian.y.atan2(cartesian.x),
    )
}

/// Returns value, bounded in range [min, max].
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
