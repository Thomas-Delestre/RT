use std::fs::File;
use std::io::{Result, Write};
use std::rc::Rc;

use crate::camera::Camera;
use crate::color;
use crate::color::Color;
use crate::common;
use crate::constants::*;
use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

pub fn draw_img() -> Result<()> {
    println!("Image dimensions: {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);

    // File
    let filename = "full_obj_pov1.ppm";
    let mut file = File::create(filename)?;
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    //World
    let _r = f64::cos(common::PI / 4.0);
    let mut world = HittableList::new();

    /*
        Placer le monde sur une sphere géante:
        let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0),100.0,material_ground,)));
    */
    
    // plane ground
    let material_ground = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.5)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, -1.0), // Un point sur le plan (y = -0.5)
        Vec3::new(0.0, 5.0, 0.0),    // Normale vers le haut (en direction de l'axe Y)
        material_ground,
    )));

    // sphere centered
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.5, -1.0),
        0.5,
        material_center,
    )));

    // sphere righted
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(2.0, 0.7, -1.0),
        0.7,
        material_right,
    )));

    
    //cube fronted
    let material_cube = Rc::new(Metal::new(Color::new(0.2, 0.1, 0.1), 0.2));
    world.add(Box::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0), // Coin inférieur
        Point3::new(0.7, 0.7, 0.7), // Coin supérieur
        material_cube,
    )));

    //cylinder on left
    let material_cylinder = Rc::new(Lambertian::new(Color::new(0.8, 0.7, 0.8)));
    world.add(Box::new(Cylinder::new(
        Point3::new(-2.0, 0.0, -1.0), // Position du centre de la base du cylindre
        1.0,                          
        0.5,                     
        material_cylinder,
    )));
    // Little cube
    let material_cube = Rc::new(Metal::new(Color::new(0.8, 0.2, 0.6), 0.5));
    world.add(Box::new(Cube::new(
        Point3::new(-0.8, 0.0, 0.0),
        Point3::new(-0.5, 0.3, 0.3),
        material_cube,
    )));
 
    // Camera
    let cam = Camera::new(
        Point3::new(0.3, 1.3, 2.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        80.0,
        ASPECT_RATIO,
    );

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
    Ok(())
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // Si nous avons dépassé la profondeur maximale de rebonds, retour à la couleur noire
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // Intersection du rayon
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec.mat.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // Couleur de l'arrière-plan (dégradé)
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.3 * (unit_direction.y() + 1.0);
    
    // Dégradé du ciel
    return (1.0 - t) * Color::new(0.4, 0.4, 0.4) + t * Color::new(0.2, 0.4, 0.6);
}



/*
horizontal = Vec3::new(viewport_width, 0.0, 0.0) → soit Vec3(3.555, 0.0, 0.0).
vertical = Vec3::new(0.0, viewport_height, 0.0) → soit Vec3(0.0, 2.0, 0.0).
focal_length = 1.0, donc on soustrait Vec3(0.0, 0.0, 1.0).

lower_left_corner = Vec3(0.0, 0.0, 0.0) - Vec3(1.7775, 0.0, 0.0) - Vec3(0.0, 1.0, 0.0) - Vec3(0.0, 0.0, 1.0)
lower_left_corner = Vec3(-1.7775, -1.0, -1.0)
 */

/*
    Normalisation de vecteur :

        Pour un pixel au milieu de l'image (u ≈ 0.5, v ≈ 0.5) {

            ower_left_corner = Vec3(-1.7775, -1.0, -1.0);
            u = 0.5006, v = 0.5011;
            horizontal = Vec3(3.555, 0.0, 0.0);
            vertical = Vec3(0.0, 2.0, 0.0);
            origin = Vec3(0.0, 0.0, 0.0);

            avant normalisation :
            dir = Vec3(-1.7775, -1.0, -1.0) + Vec3(1.778, 0.0, 0.0) + Vec3(0.0, 1.0022, 0.0)
            dir = Vec3(0.0005, 0.0022, -1.0)
            après normalisation :
            (0.0005, 0.0022, -0.99999746)

        }
    Explication incomplete, besoin d'explication concrète physique.
*/

/* Raw version of pixcel writing {
    let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
    let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
    let b = 0.9;

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    // Écrire les valeurs RGB dans le fichier
    writeln!(file, "{} {} {}", ir, ig, ib)?;
} */

/* Vec3 version of RGB storage {
    let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
    let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
    let b = 0.25;
    // utilisation batarde du Vec3 pour stoker des ensemble de 3 valeur rgb.
    // ensuite lu par la fn write_color
    // Using our new Color (alias Vec3), we’ll create a utility function to write a single pixel’s color out to the output stream.
    let pixel_color = Color::new(r, g, b);
} */
