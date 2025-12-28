use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Represents the intersection of a ray with a shape.
///
/// When a ray collides with a shape, this information is the relevant data for the intersection.
/// From top to bottom, the struct contains the point of intersection, the unit length normal
/// vector to the shape at that point, and the time value at which the intersection occurs.
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub time: f64,
}

/// This trait represents an object which can be ray traced.
pub trait Hittable {
    /// Shoot a ray at the shape.
    ///
    /// This method shoots the given ray at the shape between the given time values and returns
    /// information about a potential hit. If there was no intersection, this method returns
    /// [`None`]. If there was, a [`Some`] variant is returned holding information about the hit.
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
