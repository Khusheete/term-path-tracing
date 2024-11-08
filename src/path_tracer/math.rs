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


use rand::random;
use simple_term_renderer::math::Vec3;



#[derive(Debug, Copy, Clone)]
pub struct Interval {
    start: f64,
    end: f64
}


impl Interval {
    pub const fn new(start: f64, end: f64) -> Self {
        Self {
            start: start,
            end: end
        }
    }


    pub fn contains(&self, t: f64) -> bool {
        self.start <= t && t <= self.end
    }


    pub fn surrounds(&self, t: f64) -> bool {
        self.start < t && t < self.end
    }
}


pub fn random_unit_vec() -> Vec3 {
    let vec = 2.0 * Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, random::<f64>() - 0.5);
    vec.normalized()
}


pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let vec = random_unit_vec();
    if vec.dot(normal) > 0.0 {
        vec
    } else {
        -vec
    }
}


pub fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - 2.0 * vec.dot(normal) * normal
}


pub fn is_approx_zero(vec: Vec3) -> bool {
    vec.x.abs() < 1e-8 && vec.y.abs() < 1e-8 && vec.z.abs() < 1e-8
}


// /// Returns a normalized vector orthogonal to `vec`
// pub fn get_orthogonal(vec: Vec3) -> Vec3 {
//     if vec.y == 0.0 && vec.z == 0.0 {
//         Vec3::UNIT_Y
//     } else {
//         vec.cross(Vec3::UNIT_X).normalized()
//     }
// }


// pub struct FibHemisphereIter {
//     transformation: Mat3,
//     index: i64,
//     count: i64
// }


// impl FibHemisphereIter {
//     pub fn new(count: i64, direction: Vec3) -> Self {
//         let y = direction;
//         let x = get_orthogonal(y);
//         let z = x.cross(y);
//         Self {
//             transformation: Mat3::new(x, y, z),
//             index: 0,
//             count: count
//         }
//     }
// }


// impl Iterator for FibHemisphereIter {
//     type Item = Vec3;

//     fn next(&mut self) -> Option<Self::Item> {
//         const PHI: f64 = 3.883222077450933f64;
        
//         if self.index >= self.count {
//             return None;
//         }

//         let y = self.index as f64 / self.count  as f64;
//         let radius = (1.0 - y * y).sqrt();

//         let theta = PHI * self.index as f64;

//         let x = radius * theta.cos();
//         let z = radius * theta.sin();

//         let vec = vec3!(x, y, z);

//         self.index += 1;
//         Some(
//             self.transformation * vec
//         )
//     }
// }