use std::rc::Rc;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;
use crate::hittable::{HitRecord, Hittable};

pub struct Cylinder {
    base: Point3,         // Base center of the cylinder
    height: f64,          // Height of the cylinder
    radius: f64,          // Radius of the cylinder
    mat: Rc<dyn Material>, // Material of the cylinder
}

impl Cylinder {
    pub fn new(base: Point3, height: f64, radius: f64, mat: Rc<dyn Material>) -> Cylinder {
        Cylinder {
            base,
            height,
            radius,
            mat,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // First compute the intersection with the infinite cylinder
        let oc = r.origin() - self.base;
        let dir = r.direction();

        let a = dir.x() * dir.x() + dir.z() * dir.z();
        let half_b = oc.x() * dir.x() + oc.z() * dir.z();
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        let y_hit = oc.y() + root * dir.y(); // Check if within cylinder height
        if y_hit < 0.0 || y_hit > self.height {
            return false;
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = Vec3::new(rec.p.x() - self.base.x(), 0.0, rec.p.z() - self.base.z()).normalize();
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}