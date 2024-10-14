use std::rc::Rc;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Cube {
    min: Point3,  // Coin inférieur du cube (point minimum)
    max: Point3,  // Coin supérieur du cube (point maximum)
    mat: Rc<dyn Material>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, mat: Rc<dyn Material>) -> Cube {
        Cube { min, max, mat }
    }
}
impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // On vérifie les intersections sur chaque face du cube en utilisant la technique
        // de "slab method" ou "AABB intersection" (Axis-Aligned Bounding Box)

        // Vérifier l'intersection avec les limites sur l'axe X
        let t_x_min = (self.min.x() - r.origin().x()) / r.direction().x();
        let t_x_max = (self.max.x() - r.origin().x()) / r.direction().x();

        let (t_x_min, t_x_max) = if t_x_min > t_x_max {
            (t_x_max, t_x_min)
        } else {
            (t_x_min, t_x_max)
        };

        // Vérifier l'intersection avec les limites sur l'axe Y
        let t_y_min = (self.min.y() - r.origin().y()) / r.direction().y();
        let t_y_max = (self.max.y() - r.origin().y()) / r.direction().y();

        let (t_y_min, t_y_max) = if t_y_min > t_y_max {
            (t_y_max, t_y_min)
        } else {
            (t_y_min, t_y_max)
        };

        // Vérifier l'intersection avec les limites sur l'axe Z
        let t_z_min = (self.min.z() - r.origin().z()) / r.direction().z();
        let t_z_max = (self.max.z() - r.origin().z()) / r.direction().z();

        let (t_z_min, t_z_max) = if t_z_min > t_z_max {
            (t_z_max, t_z_min)
        } else {
            (t_z_min, t_z_max)
        };

        // Trouver les intervalles d'intersection
        let t_enter = t_x_min.max(t_y_min).max(t_z_min);
        let t_exit = t_x_max.min(t_y_max).min(t_z_max);

        // Vérifier si les intersections sont valides
        if t_enter < t_exit && t_exit > t_min && t_enter < t_max {
            rec.t = t_enter;
            rec.p = r.at(rec.t);

            // Calculer la normale à la face touchée
            if t_enter == t_x_min || t_enter == t_x_max {
                rec.set_face_normal(r, Vec3::new(1.0, 0.0, 0.0));
            } else if t_enter == t_y_min || t_enter == t_y_max {
                rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0));
            } else {
                rec.set_face_normal(r, Vec3::new(0.0, 0.0, 1.0));
            }

            rec.mat = Some(self.mat.clone());
            return true;
        }

        false
    }
}

