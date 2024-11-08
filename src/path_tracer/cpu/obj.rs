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

use crate::path_tracer::{Ray, HitInfo};
use crate::path_tracer::math::*;


pub trait Object {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitInfo>;
}


pub struct Sphere {
    position: Vec3,
    radius: f64
}


impl Sphere {
    pub fn new(position: Vec3, radius: f64) -> Self {
        Self {
            position,
            radius: radius
        }
    }
}


impl Object for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitInfo> {
        let vec_oc = self.position - ray.origin;

        let a = ray.direction.length_sq();
        let h = ray.direction.dot(vec_oc);
        let c = vec_oc.length_sq() - self.radius * self.radius;

        let delta = h * h - a * c;
        if delta < 0.0 {
            return None; // The sphere was not hit
        }


        let sqrt_delta = delta.sqrt();
        let mut root = (h - sqrt_delta) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_delta) / a;
            if !interval.surrounds(root) {
                return None; // The sphere is either too close, or too far away
            }
        }


        let surface_normal = (ray.at(root) - self.position).normalized();
        if surface_normal.dot(ray.direction) < 0.0 { // The ray comes from outside the sphere
            Some(HitInfo::front_face(
                root,
                ray.at(root),
                surface_normal
            ))
        } else {
            Some(HitInfo::back_face(
                root,
                ray.at(root),
                surface_normal
            ))
        }
    }
}


pub struct Plane {
    position: Vec3,
    normal: Vec3
}


impl Plane {

    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position: position,
            normal: normal
        }
    }
}


impl Object for Plane {
    
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitInfo> {
        let vec_oc = ray.origin - self.position;

        let a = self.normal.dot(ray.direction);

        if a == 0.0 {
            return None; // The ray is prependicular to this plane
        }

        let t = - self.normal.dot(vec_oc) / a;
        if !interval.surrounds(t) {
            return None;
        }

        if a < 0.0 { // The ray comes from above the plane
            Some(HitInfo::front_face(t, ray.at(t), self.normal))
        } else {
            Some(HitInfo::back_face(t, ray.at(t), self.normal))
        }
    }
}
