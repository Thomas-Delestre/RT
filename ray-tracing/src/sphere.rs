use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

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