use std::fmt::{write, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use crate::common;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    axe: [f64; 3],
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z: f64) -> Vec3 {
        Vec3 {  axe: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }
 
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }

    pub fn x(&self) -> f64 {
        self.axe[0]
    }

    pub fn y(&self) -> f64 {
        self.axe[1]
    }

    pub fn z(&self) -> f64 {
        self.axe[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.axe[0] * self.axe[0] + self.axe[1] * self.axe[1] + self.axe[2] * self.axe[2]
        //somme des carré
    }

    pub fn vec_length(&self) -> f64 {
        f64::sqrt(self.length_squared())
        //racine de la somme = longueur concrète
    }
    
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.vec_length()
        // divise chaque composante du vecteur par lui même pour le normaliser à ~1
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.axe[0] * v.axe[0] + u.axe[1] * v.axe[1] + u.axe[2] * v.axe[2]
    }

}

pub type Point3 = Vec3;

impl Display for Vec3 {
    // Output formatting
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.axe[0], self.axe[1], self.axe[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self::Output {
        Self::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, multiplier: f64) -> Self::Output {
        Self::new(self.x() * multiplier, self.y() * multiplier, self.z() * multiplier)
    }
    
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, divisor: f64) -> Self::Output {
        Self::new(self.x() / divisor, self.y() / divisor, self.z() / divisor)
    }
}

// pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
//     Vec3::new(
//         u.axe[1] * v.axe[2] - u.axe[2] * v.axe[1],
//         u.axe[2] * v.axe[0] - u.axe[0] * v.axe[2],
//         u.axe[0] * v.axe[1] - u.axe[1] * v.axe[0],
//     )
// }

