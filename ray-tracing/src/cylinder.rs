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
        let mut hit_anything = false;
        let mut closest_t = t_max;

        // Check for intersection with the cylindrical surface
        let oc = r.origin() - self.base;
        let dir = r.direction();

        let a = dir.x() * dir.x() + dir.z() * dir.z();
        let half_b = oc.x() * dir.x() + oc.z() * dir.z();
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();

            // Check the smaller root
            let mut root = (-half_b - sqrt_d) / a;
            let y_hit = oc.y() + root * dir.y();
            if root > t_min && root < closest_t && y_hit >= 0.0 && y_hit <= self.height {
                closest_t = root;
                hit_anything = true;

                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = Vec3::new(rec.p.x() - self.base.x(), 0.0, rec.p.z() - self.base.z()).normalize();
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.mat.clone());
            }

            // Check the larger root
            root = (-half_b + sqrt_d) / a;
            let y_hit = oc.y() + root * dir.y();
            if root > t_min && root < closest_t && y_hit >= 0.0 && y_hit <= self.height {
                closest_t = root;
                hit_anything = true;

                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = Vec3::new(rec.p.x() - self.base.x(), 0.0, rec.p.z() - self.base.z()).normalize();
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.mat.clone());
            }
        }

        // Check for intersection with the bottom cap
        let t_cap_bottom = (self.base.y() - r.origin().y()) / r.direction().y();
        if t_cap_bottom > t_min && t_cap_bottom < closest_t {
            let p_cap = r.at(t_cap_bottom);
            let d = (p_cap.x() - self.base.x()).powi(2) + (p_cap.z() - self.base.z()).powi(2);
            if d <= self.radius * self.radius {
                closest_t = t_cap_bottom;
                hit_anything = true;

                rec.t = t_cap_bottom;
                rec.p = p_cap;
                rec.set_face_normal(r, Vec3::new(0.0, -1.0, 0.0)); // Normal points downward
                rec.mat = Some(self.mat.clone());
            }
        }

        // Check for intersection with the top cap
        let t_cap_top = (self.base.y() + self.height - r.origin().y()) / r.direction().y();
        if t_cap_top > t_min && t_cap_top < closest_t {
            let p_cap = r.at(t_cap_top);
            let d = (p_cap.x() - self.base.x()).powi(2) + (p_cap.z() - self.base.z()).powi(2);
            if d <= self.radius * self.radius {
                closest_t = t_cap_top;
                hit_anything = true;

                rec.t = t_cap_top;
                rec.p = p_cap;
                rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0)); // Normal points upward
                rec.mat = Some(self.mat.clone());
            }
        }

        hit_anything
    }
}
