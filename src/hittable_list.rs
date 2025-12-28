use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

/// A simple wrapper structure for a list of hittable objects.
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    /// Create an empty [`HittableList`].
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Create a hittable list from a single object.
    pub fn from_hittable(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Push a new hittable into the list.
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Casting a ray at the scene, we want to know the very first thing it would hit.

        let mut closest_hit: Option<HitRecord> = None;
        for object in &self.objects {
            let closest_t = closest_hit.map(|rec| rec.time).unwrap_or(t_max);
            
            // There is no need to trace the ray further than this value of t, since it already hit
            // something there. We need only check to see if it hit something closer.
            if let Some(hit_info) = object.hit(ray, t_min, closest_t) {
                closest_hit = Some(hit_info);
            }
        }
        closest_hit
    }
}
