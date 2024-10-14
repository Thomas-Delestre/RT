# Ray Tracing Project

Ce projet implémente un moteur de ray tracing en Rust, permettant de générer des images réalistes en simulant la manière dont la lumière interagit avec les objets dans une scène 3D.

### Fonctionnalités principales
* Géométrie : Support de formes primitives telles que des sphères, des cubes, des cylindres, et des plans.
* Matériaux : Simulation de différents matériaux, comme le Lambertian (diffus) et le Metal (réfléchissant).
* Caméra : Une caméra paramétrable pour observer la scène sous différents angles avec contrôle du champ de vision.
* Anti-aliasing : Utilisation de l’échantillonnage multiple par pixel pour lisser les bords des objets et améliorer la qualité de l'image.
* Limite de rebond : Contrôle de la profondeur maximale de récursion des rayons pour limiter les calculs.

### Constantes principales
    Les constantes comme la résolution de l'image, le nombre d'échantillons par pixel, et le ratio d'aspect sont configurables dans un fichier séparé (constants.rs).