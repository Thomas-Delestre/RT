use crate::vec3::{Point3, Vec3};
 
#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
 
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
 
    pub fn origin(&self) -> Point3 {
        self.orig
    }
 
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
 
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
    /*
    In ray tracing, we define a Ray as a mathematical representation of a path through 3D space. This can be expressed as:
    Position(t)= Origin+t⋅Direction
    Position(t): The 3D point along the ray.
    Origin: The starting point of the ray.
    Direction: A vector indicating the ray's path.
    t: A parameter that moves along the ray; changing its value lets you traverse the line in both directions.
    */