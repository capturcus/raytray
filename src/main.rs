extern crate cgmath;
extern crate image;

use cgmath::prelude::*;
use cgmath::Vector3;
use image::*;
use std::fs::{File, OpenOptions};

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3<f64> = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn to_rgba(c: &Color) -> Rgba<u8> {
    Rgba::from_channels((gamma_encode(c.red) * 255.0) as u8,
                        (gamma_encode(c.green) * 255.0) as u8,
                        (gamma_encode(c.blue) * 255.0) as u8,
                        255)
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Vector3{x: 0., y: 0., z: 0.},
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }.normalize(),
        }
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color))
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    /*let mut image_file =
        OpenOptions::new().write(true).truncate(true).create(true).open("image.png").unwrap();*/
    img.save("image.png").unwrap();
}
