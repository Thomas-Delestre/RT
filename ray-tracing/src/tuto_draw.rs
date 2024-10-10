use crate::color;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::fs::File;
use std::io::{Result, Write};
use crate::sphere::Sphere;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::common;
use crate::camera::Camera;



pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 600;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
pub const SAMPLES_PER_PIXEL: i32 = 100;
pub const MAX_DEPTH: i32 = 50;

pub fn draw_img() -> Result<()> {
    
    //World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Box::new(Sphere::new(Point3::new(1.5, 0.2, -2.0), 0.5)));
    
    // Camera
    let cam = Camera::new();

    // File
    let filename1 = "tuto_pict.ppm";
    let mut file1 = File::create(filename1)?;

    writeln!(file1, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

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
            color::write_color(&mut file1, pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
    Ok(())
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    // Ray intersection 
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let direction = rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);
    }

    // Couleur de l'arrière-plan (dégradé)
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0); // Dégradé du ciel
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
    //utilisation batarde du Vec3 pour stoker des ensemble de 3 valeur rgb.
    //ensuite lu par la fn write_color
    // Using our new Color (alias Vec3), we’ll create a utility function to write a single pixel’s color out to the output stream.
    let pixel_color = Color::new(r, g, b);
} */


