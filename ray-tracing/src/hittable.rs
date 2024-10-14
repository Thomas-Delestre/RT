use std::rc::Rc;
 
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
 
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3, // les point d'untersections
    pub normal: Vec3, // vecteur pointant vers l'extérieur de la surface qu'elle rencontre (direction)
    pub mat: Option<Rc<dyn Material>>, 
    pub t: f64, // distance entre intersection et la cam
    pub front_face: bool,
}
//Cette structure ne stocke que l'intersection la plus proche (et donc la plus pertinente pour l'affichage)
 
impl HitRecord {

    pub fn new() -> HitRecord {
        Default::default()
        // implémentation de valeur par défaut pour chaque champ. donc 
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
 
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    // rayon , 0.0 (camera), INFINI, le HitRecord créé par défault

}