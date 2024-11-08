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


pub mod rid;
pub mod cpu;


mod math;

use simple_term_renderer::img::Image;
use simple_term_renderer::math::*;


pub trait PTRenderer {
    fn render(&self, camera: &Camera, target: &mut Image);
}


pub struct Camera {
    pub position: Vec3,
    pub focal_length: f64
}



impl Camera {

    pub fn new(position: Vec3, focal_length: f64) -> Self {
        Self {
            position: position,
            focal_length: focal_length
        }
    }
}


pub struct HitInfo {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub front_face: bool
}


impl HitInfo {
    pub fn new(distance: f64, position: Vec3, normal: Vec3, front_face: bool) -> HitInfo {
        Self {
            distance: distance,
            position: position,
            normal: normal,
            front_face: front_face
        }
    }


    pub fn front_face(distance: f64, position: Vec3, out_normal: Vec3) -> HitInfo {
        Self::new(distance, position, out_normal, true)
    }


    pub fn back_face(distance: f64, position: Vec3, out_normal: Vec3) -> HitInfo {
        Self::new(distance, position, -out_normal, false)
    }
}


// TODO: privates
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}


impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction
        }
    }

    
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}