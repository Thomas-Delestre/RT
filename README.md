# Ray Tracing Project

Ce projet implémente un moteur de ray tracing en Rust, permettant de générer des images réalistes en simulant la manière dont la lumière interagit avec les objets dans une scène 3D.

Ce projet a été réaliser en suivant un tutoriel complet : https://the-ray-tracing-road-to-rust.vercel.app/

### Fonctionnalités principales
* Géométrie : Support de formes primitives telles que des sphères, des cubes, des cylindres, et des plans.
* Matériaux : Simulation de différents matériaux, comme le Lambertian (diffus) et le Metal (réfléchissant).
* Caméra : Une caméra paramétrable pour observer la scène sous différents angles avec contrôle du champ de vision.
* Anti-aliasing : Utilisation de l’échantillonnage multiple par pixel pour lisser les bords des objets et améliorer la qualité de l'image.
* Limite de rebond : Contrôle de la profondeur maximale de récursion des rayons pour limiter les calculs.

### Constantes principales
Les constantes comme la résolution de l'image, le nombre d'échantillons par pixel, et le ratio d'aspect sont configurables dans un fichier séparé **constants.rs**.

# Audit
Retrouvez les 4 images necessaire à l'audit dans le dossier audit_img._
### Using the ray tracer construct any scene you want, including at least one of all objects. (this can take a while to render, so in the meantime you can skip the first two questions and answer the following ones)
* Does the image correspond to the scene you created?
    _ Oui, l'image affiche correctement les objets ajouté danns le monde. Ajouté un petit cube clair en métal semi-réfléchissant en le plaçant a gauche du premier:
        ```
        let material_cube = Rc::new(Metal::new(Color::new(0.7, 0.9, 0.8), 0.5));

        world.add(Box::new(Cube::new(

            Point3::new(-0.4, 0.0, -0.4),

            Point3::new(-0.1, 0.3, -0.1),

            material_cube,
            
        )));
        ```
* Is it possible for you to reduce the resolution of the output image?
    _ Dans le fichier **constants.rs** en modifiant la valeur de **IMAGE_WIDTH**. Vous modifiez aussi **IMAGE_HEIGHT** grace au ratio. Ainsi la resolution est modifié
### Move the camera and render the same scene.
* Does the image correspond to the same scene, but from a different perspective?
    _ Pour bouger la camera, modifié ces valeurs dans le fichier **tuto_draw** :
        ```
        let cam = Camera::new(

        Point3::new(0.3, 1.3, 2.0),  // Position de la camera

        Point3::new(0.0, 0.0, 0.0),  // Position que la camera regarde

        Vec3::new(0.0, 1.0, 0.0),    // Modifie son inclinaison sur les 3 axes

        80.0,                        // Champ de vision vertical

        ASPECT_RATIO,

        );
        ```

* Did the student provide 4 .ppm pictures?
    * Vous pouvez générer autant d'image que vous le souhaitez en modifiant le nom de fichier créé lors de la génération en modifiant la var **filename** dans le fichier **tuto_draw**.

* Does one of these images consist of a scene with a sphere?
    * Ouvrez l'image **sphere_scene_png**.

* Does one of these images consist of a scene with a flat plane and a cube with lower brightness than in the sphere image?
    * Ouvrez l'image **cube_scene_darker.png**

* Does one of these images consist of a scene with one of each of all the objects (one cube, one sphere, one cylinder and one flat plane)?
    * Ouvrez l'image **full_obj_pov1.png**

* Does one of these images consist of a scene like the previous one, but with the camera in another position (thus generating the same image from a different perspective)?
    * Ouvrez l'image **full_obj_pov2.png**

* Considering all of the previous pictures, can you see shadows from the objects?
    * Elle sont particulièrement remarquable en dessous des sphères. Même si l'ombrage général est assez imperseptible.

* Did the student provide clear documentation for the ray tracer on how to use it (create elements, change brightness and move the camera)?
    * create elements -> Se référer au premier code fourni.
    * change brightness -> Dans le fichier **tuto_draw** sur la ligne du return final, augmenter les 3 valeurs des Color::new pour obscursir le rendu.
    ```
    return (1.0 - t) * Color::new(0.4, 0.4, 0.4) + t * Color::new(0.2, 0.4, 0.6);
    ```
    * move camera -> Se référer au explication lors de la vérification des déplacement de camera.