use std::rc::Rc;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;
use crate::hittable::{HitRecord, Hittable};

pub struct Plane {
    point: Point3,  // Un point sur le plan
    normal: Vec3,   // La normale du plan
    mat: Rc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, mat: Rc<dyn Material>) -> Plane {
        Plane {
            point,
            normal: Vec3::unit_vector(normal),  // On normalise la normale pour s'assurer qu'elle est de longueur 1
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(self.normal, r.direction());
        
        // Vérifie si le rayon est parallèle au plan
        if denom.abs() > 1e-6 {  // Tolérance pour éviter les erreurs de flottants
            let t = Vec3::dot(self.point - r.origin(), self.normal) / denom;
            
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = r.at(t);
                rec.set_face_normal(r, self.normal);
                rec.mat = Some(self.mat.clone());
                return true;
            }
        }
        false
    }
}