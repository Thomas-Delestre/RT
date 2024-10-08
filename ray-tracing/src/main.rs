mod tuto_draw;
mod color;
mod ray;
mod sphere;
mod vec3;
mod hittable;
mod hittable_list;
mod common;

use tuto_draw::{draw_img, IMAGE_WIDTH, IMAGE_HEIGHT}; // Import the constants

fn main() {
    println!("Image dimensions: {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT); // Example usage
    let _ = draw_img(); // Call the draw_img function
}


/*
Allons y étape par étape. EN parlant concrètement, avec le minimum de formule possible et un maximum d'exemple concret: 
_ J'ai un écran de 800x450, affichant une espace 3D infini, dont les distance entre 2 point se calcul en unité. Unité définie par les proportion (ici 16.0/9.0) 
_ J'ai ma camera, a une distance en unité de espace 3D. Dans cette espace 3D.
_ J'ai une sphere, dont le centre est a une distance en unité de ma camera est dont le rayon de 0,5
_ L'affichage de cette sphère se fait, par ray tracing, et si un point du vecteur de ray tracing intercepte un point se trouvant a une distance de 0.5 unité du centre du cercle, il affiche la couleur de la sphère.
_ Maintenant pour les ombrages. J'ai uniquement envie de savoir, où est défini la position de la source de lumière.
*/