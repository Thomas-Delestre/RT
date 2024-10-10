use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
 
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

/*
Le systeme de box, permet de stocker des obj de taille ou type diff
<dyn Hittable> pour uniquement indiquer que tout les obj seront dynamique et auront le trait Hittable.
Box<dyn Hittable> est utilisé pour gérer une liste d'objets différents qui implémentent tous le trait Hittable. 
Comme chaque type (par exemple, Sphere) peut avoir une taille différente,
tu utilises des Box pour stocker ces objets dans un vecteur, car un vecteur nécessite que tous les éléments aient la même taille. 
Le dyn est nécessaire car les types concrets des objets ne sont pas connus à la compilation, mais sont résolus dynamiquement au moment de l'exécution.
*/
 
impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    } // établi un monde prêt a etre rempli
 
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
 
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
 
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
            // permet de d'enregistrer dans HitRecord, le point de contact le plus proche de la cam
        }
 
        hit_anything
    }
}