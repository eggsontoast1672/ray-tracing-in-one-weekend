use raytracing::math::interval::Interval;
use raytracing::math::ray::Ray;
use raytracing::math::vec3::{Point3, Vec3};

/// Represents the intersection of a ray with a shape.
///
/// When a ray collides with a shape, this information is the relevant data for the intersection.
#[derive(Clone, Copy)]
pub struct HitRecord {
    /// The point at which the intersection occurred.
    pub _point: Point3,

    /// The normal vector to the surface at the point of intersection.
    pub normal: Vec3,

    /// The value of t for which the ray intersected the surface.
    pub time: f64,

    /// True if the ray intersected the front face of the surface, false otherwise.
    pub front_face: bool,
}

impl HitRecord {
    /// Set the face normal.
    ///
    /// This method sets the face normal with the given ray and normal information. If the dot
    /// product of the ray's direction with the surface normal is negative, then they are pointing
    /// in different directions. Surface normals always point outward, so the ray intersected the
    /// front of the face.
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

/// This trait represents an object which can be ray traced.
pub trait Hittable {
    /// Shoot a ray at the shape.
    ///
    /// This method shoots the given ray at the shape between the given time values and returns
    /// information about a potential hit. If there was no intersection, this method returns
    /// [`None`]. If there was, a [`Some`] variant is returned holding information about the hit.
    fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord>;
}
