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
    let material_ground = Rc::new(Lambertian::new(Color::new(0.05, 0.05, 0.05)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, -1.0), // Un point sur le plan (y = -0.5)
        Vec3::new(0.0, 5.0, 0.0),    // Normale vers le haut (en direction de l'axe Y)
        material_ground,
    )));

    // sphere centered
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, -2.5),
        1.0,
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
    let material_cube = Rc::new(Metal::new(Color::new(0.4, 0.8, 0.6), 0.2));
    world.add(Box::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0), // Coin inférieur
        Point3::new(0.7, 0.7, 0.7), // Coin supérieur
        material_cube,
    )));

    //cylinder on left
    let material_cylinder = Rc::new(Metal::new(Color::new(0.5, 0.9, 0.3), 0.2));
    world.add(Box::new(Cylinder::new(
        Point3::new(-2.0, -1.0, -1.0),
        2.5, // Height
        0.5, // Radius
        material_cylinder,
    )));
    
    // Camera
    let cam = Camera::new(
        Point3::new(1.8, 1.7, 1.2),
        Point3::new(0.4, 0.4, 0.0),
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
    return (2.0 - t) * Color::new(0.4, 0.4, 0.4) + t * Color::new(0.2, 0.4, 0.6);
}
