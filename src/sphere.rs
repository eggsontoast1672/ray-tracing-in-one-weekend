use std::cmp::Ordering;

use raytracing::math::interval::Interval;
use raytracing::math::ray::Ray;
use raytracing::math::vec3::Point3;

use crate::hittable::{HitRecord, Hittable};

/// Represents a ray traceable sphere.
///
/// A sphere is determined by its center point and radius, but I am sure I did not need to tell you
/// that. I just wanted to fill up the doc comment with something. The radius should ideally never
/// be negative, but that can be easily circumvented by just setting the radius field to something
/// negative.
#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    /// Create a new sphere.
    ///
    /// This function creates a new sphere with the given center and radius. The radius is either
    /// negative or not comperable to 0 (e.g. infinity or NaN), then the radius is initialized to
    /// 0.
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: match radius.partial_cmp(&0.0) {
                Some(Ordering::Greater) => radius,
                _ => 0.0,
            },
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // We need to find a root that occurs within the allowed time values. If the the ray
        // intersects the sphere, there is no guarantee that the intersection will occur in the
        // bounds we desire.
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        // Filling up the hit record with information. This is not a good comment.
        let hit_point = ray.at(root);
        let mut record = HitRecord {
            _point: hit_point,
            normal: (hit_point - self.center) / self.radius,
            time: root,
            front_face: false,
        };

        record.set_face_normal(ray, record.normal);
        Some(record)
    }
}
