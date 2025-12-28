use crate::math::vec3::{Point3, Vec3};

/// Represents a ray.
///
/// This structure holds all of the information necessary to cast a ray. The origin is the start of
/// the ray, and the direction is that along which the ray travels. The idea is to compute the path
/// of the ray at a certain point in time using a linear equation in these two factors.
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// Create a zero ray.
    ///
    /// This function creates a ray with zero origin and magnitude. It is a trivial ray that does
    /// not have much use.
    pub const fn zero() -> Self {
        Self {
            origin: Point3::zero(),
            direction: Vec3::zero(),
        }
    }

    /// Create a ray with specified origin and direction.
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Compute the ray's position at a given time.
    ///
    /// This method is the meat and potatoes of the ray structure. It represents the ray's position
    /// as we move through time, which is essentially what it means to cast the ray.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
