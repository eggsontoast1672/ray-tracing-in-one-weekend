use raytracing::color::Color;
use raytracing::math::ray::Ray;
use raytracing::math::{self, Vec3};

use crate::hittable::HitRecord;

pub struct Reflection {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<Reflection>;
}

/// A diffuse (matte) material.
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: HitRecord) -> Option<Reflection> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        Some(Reflection {
            ray: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<Reflection> {
        let reflected = math::reflect(ray.direction, hit.normal);

        Some(Reflection {
            ray: Ray::new(hit.point, reflected),
            attenuation: self.albedo,
        })
    }
}
