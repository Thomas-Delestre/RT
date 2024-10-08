use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(file: &mut File, pix_col: Color) {
    // Write the translated [0, 255] value of each color component
    let r = (255.999 * pix_col.x()) as i32;
    let g = (255.999 * pix_col.y()) as i32;
    let b = (255.999 * pix_col.z()) as i32;
    writeln!(file, "{} {} {}", r, g, b).expect("writing color");
}