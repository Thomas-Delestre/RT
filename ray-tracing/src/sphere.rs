use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::hittable::{HitRecord, Hittable};


pub struct Sphere {
    center: Point3,
    radius: f64,
}
 
impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
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
 
        let sqrt_d = f64::sqrt(discriminant);
 
        // Find the nearest root that lies in the acceptable range
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
        true
    }
}

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center; // oeil de ma cam - centre du cercle = connaitre distance de la cam au cercle
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(oc, r.direction());
    // à quelle distance le rayon se rapproche ou s'éloigne du centre de la sphère.
    let c = oc.length_squared() - radius * radius;
    //Déterminer si la caméra se trouve à une distance du centre de la sphère qui pourrait potentiellement permettre au rayon d'intercepter la sphère.
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0 // valeur conventionnelle, pas d'intersection
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
        //en cas de multiple intersection, on récupère le point le plus proche de la cam, pour afficher celui-ci uniquement.
    }
}

/*
Si le discriminant est positif, il y a deux solutions possibles (le rayon traverse la sphère à deux points différents).
Si le discriminant est égal à 0, le rayon touche la sphère en un seul point (tangence). 
Si le discriminant est négatif, il n'y a pas de solutions réelles, donc pas d'intersection (le rayon ne touche pas la sphère).
*/