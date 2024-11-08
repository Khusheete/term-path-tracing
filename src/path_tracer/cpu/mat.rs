/*
Copyright 2024 Souchet Ferdinand

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the “Software”), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/


use simple_term_renderer::math::Vec3;

use crate::{HitInfo, Ray};

use super::{is_approx_zero, random_on_hemisphere, random_unit_vec, reflect};



pub trait Material {
    fn scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> (Vec3, Ray);
}



pub struct Lambertian {
    albedo: Vec3
}


impl Lambertian {

    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: albedo
        }
    }
}


impl Material for Lambertian {
    fn scatter(&self, _in_ray: &Ray, hit_info: &HitInfo) -> (Vec3, Ray) {
        let scattered = random_unit_vec() + random_on_hemisphere(hit_info.normal);
        if is_approx_zero(scattered) {
            (self.albedo, Ray::new(hit_info.position, hit_info.normal))
        } else {
            (self.albedo, Ray::new(hit_info.position, scattered))
        }
    }
}


pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}


impl Metal {

    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: fuzz
        }
    }
}


impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> (Vec3, Ray) {
        (
            self.albedo,
            Ray::new(
                hit_info.position,
                reflect(in_ray.direction, hit_info.normal) + self.fuzz * random_unit_vec()
            )
        )
    }
}