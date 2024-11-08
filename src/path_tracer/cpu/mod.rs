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


mod obj;
mod mat;

use std::collections::HashMap;
use std::f64::consts::TAU;

use simple_term_renderer::img::Color;
use simple_term_renderer::{img::Image, vec2, vec3};
use simple_term_renderer::math::*;
use super::math::*;

use crate::rid::{Rid, RidOwner};
use crate::{Camera, HitInfo, PTRenderer, Ray};


use obj::*;
use mat::*;


pub struct CpuRenderingDevice {
    objects: RidOwner<Box<dyn Object>>,

    materials: RidOwner<Box<dyn Material>>,
    default_material: Rid,
    object_materials: HashMap<Rid, Rid>,

    pub max_light_bounce: i64,
    pub pixel_sample_count: i64
}


impl CpuRenderingDevice {

    pub fn new(max_light_bounce: i64, pixel_sample_count: i64) -> Self {
        let mut materials: RidOwner<Box<dyn Material>> = RidOwner::new();
        let default_material = materials.add(Box::new(Lambertian::new(vec3!(0.5, 0.5, 0.5))));

        Self {
            objects: RidOwner::new(),
            materials: materials,
            object_materials: HashMap::new(),
            default_material: default_material,
            max_light_bounce: max_light_bounce,
            pixel_sample_count: pixel_sample_count
        }
    }

    pub fn create_sphere(&mut self, position: Vec3, radius: f64) -> Rid {
        let rid = self.objects.add(Box::new(
            Sphere::new(position, radius)
        ));
        self.object_set_material(rid, self.default_material);
        rid
    }


    pub fn create_plane(&mut self, normal: Vec3, position: Vec3) -> Rid {
        let rid = self.objects.add(Box::new(
            Plane::new(normal, position)
        ));
        self.object_set_material(rid, self.default_material);
        rid
    }


    pub fn create_lambertial_material(&mut self, albedo: Color) -> Rid {
        self.materials.add(Box::new(
            Lambertian::new(albedo.get_raw_vec3f()))
        )
    }


    pub fn create_metal_material(&mut self, albedo: Color, fuzz: f64) -> Rid {
        self.materials.add(Box::new(
            Metal::new(albedo.get_raw_vec3f(), fuzz)
        ))
    }


    pub fn object_set_material(&mut self, obj_rid: Rid, mat_rid: Rid) {
        self.object_materials.entry(obj_rid)
            .and_modify(|entry| {*entry = mat_rid})
            .or_insert(mat_rid);
    }


    pub fn remove_object(&mut self, rid: Rid) {
        self.objects.remove(rid);
    }


    pub fn remove_material(&mut self, rid: Rid) {
        self.materials.remove(rid);
    }


    fn ray_color(&self, ray: &Ray, bounce_count: i64) -> Vec3 {
        if bounce_count > self.max_light_bounce { // The light would not stop bouncing
            return Vec3::ZERO;
        }

        let interval = &Interval::new(0.001, f64::INFINITY); // should be in rendering context or camera (far/near)
        let mut hit: Option<(HitInfo, &Rid)> = None;

        // Process object hits
        for (rid, obj) in self.objects.rid_value_iter() {
            if let Some(obj_hit) = (*obj).hit(ray, &interval) {
                if !interval.contains(obj_hit.distance) {
                    continue;
                }
                if let Some((hit_info, _)) = &hit {
                    if obj_hit.distance < hit_info.distance {
                        hit = Some((obj_hit, rid));
                    }
                } else {
                    hit = Some((obj_hit, rid));
                }
            }
        }
        

        // Process object material if there was a hit
        if let Some((hit_info, obj_rid)) = hit {
            let mat_rid = self.object_materials.get(obj_rid).unwrap();
            let mat = self.materials.get(*mat_rid).or(self.materials.get(self.default_material)).unwrap();
            let (attenuation, bounce_ray) = mat.scatter(ray, &hit_info);

            let env_contrib = self.ray_color(&bounce_ray, bounce_count + 1);

            return vec3!(
                attenuation.x * env_contrib.x,
                attenuation.y * env_contrib.y,
                attenuation.z * env_contrib.z
            );
        }
    
        // Get sky color
        let ray_dir = ray.direction.normalized();
        let sun_dir = vec3!(0.6, 0.6, 0.35).normalized();

        if ray_dir.dot(sun_dir) > (TAU/25.0).cos() {
            return 1.8 * vec3!(0.95, 0.9, 0.6);
        } else {
            let a = 0.5 * (ray_dir.y + 1.0);
            return 0.32 * (a * vec3!(0.5, 0.7, 1.0) + (1.0 - a) * vec3!(1.0, 1.0, 1.0));
        }
    }
}


impl PTRenderer for CpuRenderingDevice {
    fn render(&self, camera: &Camera, target: &mut Image) {
        let size = target.size();
        let aspect_ratio: f64 = size.x as f64 / size.y as f64;
    
        let mut viewport_size = vec2!(0.0, 2.0);
        viewport_size.x = viewport_size.y * aspect_ratio;
    
        let viewport_u = vec3!(viewport_size.x, 0.0, 0.0);
        let viewport_v = vec3!(0.0, -viewport_size.y, 0.0);
    
        let pixel_delta_u = viewport_u / size.x as f64;
        let pixel_delta_v = viewport_v / size.y as f64;
    
        let pixel_top_left = camera.position
            - vec3!(0.0, 0.0, camera.focal_length) - 0.5 * (viewport_u + viewport_v);
    
    
        for j in 0..size.y {
            for i in 0..size.x {
                // Get pixel ray
                let pixel_source = pixel_top_left + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
                let ray_direction = pixel_source - camera.position;
    
                let ray = Ray::new(camera.position, ray_direction);

                // Sample pixel color
                let mut pixel_color = Vec3::ZERO;

                for _sample in 0..self.pixel_sample_count {
                    pixel_color += self.ray_color(&ray, 0);
                }

                pixel_color /= self.pixel_sample_count as f64;

                // Apply gamma correction
                pixel_color.x = pixel_color.x.sqrt().clamp(0.0, 1.0);
                pixel_color.y = pixel_color.y.sqrt().clamp(0.0, 1.0);
                pixel_color.z = pixel_color.z.sqrt().clamp(0.0, 1.0);

                // Write pixel
                target.point((i, j), Color::raw_vec3_rgb(pixel_color));
            }
        }
    }
}

