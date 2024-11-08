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


extern crate simple_term_renderer;

mod path_tracer;

use std::sync::{Arc, Mutex};
use std::time;

use simple_term_renderer::{img::{Color, Image}, input::Input, *};
use math::*;
use rds::Renderer;

use path_tracer::{cpu, *};


fn main() {
    let rdr = Renderer::get();

    // Setup variables
    let size = Renderer::get_size();
    let mut canvas = Image::new(size);
    
    // Create camera
    let camera = Camera::new(vec3!(0.0, 0.0, 0.0), 1.0);

    let mut cpu_path_tracer = cpu::CpuRenderingDevice::new(3, 1000);


    // Setup world
    let red_ball = cpu_path_tracer.create_metal_material(Color::raw_rgb(0.8, 0.4, 0.4), 0.2);
    let default_ball = cpu_path_tracer.create_lambertial_material(Color::raw_rgb(0.4, 0.4, 0.4));
    let grass = cpu_path_tracer.create_lambertial_material(Color::raw_rgb(0.2, 0.8, 0.2));

    let sphere1 = cpu_path_tracer.create_sphere(vec3!(-1.0, 0.0, -1.8), 0.5);
    cpu_path_tracer.object_set_material(sphere1, default_ball);

    let sphere2 = cpu_path_tracer.create_sphere(vec3!(0.0, 0.0, -2.0), 0.5);
    cpu_path_tracer.object_set_material(sphere2, red_ball);

    let sphere3 = cpu_path_tracer.create_sphere(vec3!(1.0, 0.0, -1.8), 0.5);
    cpu_path_tracer.object_set_material(sphere3, default_ball);

    let plane = cpu_path_tracer.create_plane(vec3!(0.0, -0.5, 0.0), Vec3::UNIT_Y);
    cpu_path_tracer.object_set_material(plane, grass);
    
    // Render image
    let path_tracer_start = time::Instant::now();
    cpu_path_tracer.render(&camera, &mut canvas);
    let time = path_tracer_start.elapsed().as_micros();

    rdr.begin_draw();
    rdr.draw_whole_image(Arc::new(Mutex::new(canvas)), (0, 0));
    rdr.print_blended_text_raw(
        &format!("{} μs", time), (1, 1)
    );
    rdr.end_draw();

    // Cleanup
    cpu_path_tracer.remove_object(sphere1);
    cpu_path_tracer.remove_object(sphere2);
    cpu_path_tracer.remove_object(sphere3);
    cpu_path_tracer.remove_object(plane);

    cpu_path_tracer.remove_material(red_ball);
    cpu_path_tracer.remove_material(grass);

    // Wait for input and exit
    Input::get().get_event_blocking();
    Renderer::exit();
}