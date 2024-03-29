use crate::material::Material;
use crate::object::Object;
use crate::ray::{HitRec, Ray};
use crate::Vec3;

use rand::prelude::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec<'_>> {
        let oc = ray.orig - self.center;
        let a = ray.dire.dot(&ray.dire);
        let b = oc.dot(&ray.dire);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminent = b * b - a * c;
        if discriminent >= 0.0 {
            let temp = (-b - discriminent.sqrt()) / a;
            if (temp <= t_max) & (temp >= t_min) {
                let point = ray.point_at_parameter(temp);
                let norm = (point - self.center) / self.radius;
                return Some(HitRec {
                    t: temp,
                    p: point,
                    norm,
                    material: &self.material,
                });
            }
            let temp = (-b + discriminent.sqrt()) / a;
            if (temp <= t_max) & (temp >= t_min) {
                let point = ray.point_at_parameter(temp);
                let norm = (point - self.center) / self.radius;
                return Some(HitRec {
                    t: temp,
                    p: point,
                    norm,
                    material: &self.material,
                });
            }
            None
        } else {
            None
        }
    }
}

pub(crate) fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        // Random taken from a cube of 2x2x2 centered on 1x1x1
        let mut p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0;
        p -= Vec3::from_element(1.0);
        if p.dot(&p) < 1.0 {
            // Is it in a sphere?
            return p;
        }
    }
}
