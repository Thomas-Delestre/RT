use std::rc::Rc;

use crate::ray::Ray;
use crate::material::Material;
use crate::vec3::{Point3, Vec3};
use crate::hittable::{HitRecord, Hittable};


pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>
}
 
impl Sphere {
    pub fn new(cen: Point3, r: f64, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat,
        }
    }
}
 
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
 
        // On va sélectionner grace a ce calcul, l'intersection la plus proche de la camera puisque le reste des intersetion sera en back face
        let sqrt_d = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
 
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}

/*
Si le discriminant est positif, il y a deux solutions possibles (le rayon traverse la sphère à deux points différents).
Si le discriminant est égal à 0, le rayon touche la sphère en un seul point (tangence). 
Si le discriminant est négatif, il n'y a pas de solutions réelles, donc pas d'intersection (le rayon ne touche pas la sphère).
*/